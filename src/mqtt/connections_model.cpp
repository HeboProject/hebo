// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "connections_model.h"

#include <QDebug>
#include <QDir>
#include <QStandardPaths>

#include "base/random.h"

namespace hebo {
namespace {

constexpr const char* kId = "id";
constexpr const char* kName = "name";
constexpr const char* kClientId = "clientId";
constexpr const char* kProtocol = "protocol";
constexpr const char* kHost = "host";
constexpr const char* kPort = "port";
constexpr const char* kQoS = "qos";
constexpr const char* kUsername = "username";
constexpr const char* kPassword = "password";
constexpr const char* kTls = "tls";
constexpr const char* kCleanSession = "cleanSession";
constexpr const char* kDescription = "description";
constexpr const char* kConnectionState = "connectionState";

QString getJsonFile() {
  const QStringList dirs = QStandardPaths::standardLocations(QStandardPaths::AppConfigLocation);
  Q_ASSERT(!dirs.isEmpty());
  QDir dir(dirs.first());
  dir.cdUp();
  dir.mkpath(".");
  return dir.absoluteFilePath("connections.json");
}

}  // namespace

ConnectionsModel::ConnectionsModel(QObject* parent)
    : QAbstractListModel(parent),
      conn_file_(getJsonFile()) {
  qRegisterMetaType<QoS>("QoS");
  qRegisterMetaType<ConnectionState>("ConnectionState");

  // Load connections on startup.
  this->loadConnInfo();
}

int ConnectionsModel::rowCount(const QModelIndex& parent) const {
  Q_UNUSED(parent);
  return this->configs_.length();
}

QVariant ConnectionsModel::data(const QModelIndex& index, int role) const {
  if (!index.isValid()) {
    return {};
  }

  const ConnectConfig& info = this->configs_.at(index.row());
  switch (role) {
    case kIdRole: {
      return info.id;
    }
    case kNameRole: {
      return info.name;
    }
    case kClientIdRole: {
      return info.client_id;
    }
    case kProtocolRole: {
      return info.protocol;
    }
    case kHostRole: {
      return info.host;
    }
    case kPortRole: {
      return info.port;
    }
    case kQoSRole: {
      return static_cast<int>(info.qos);
    }
    case kUsernameRole: {
      return info.username;
    }
    case kPasswordRole: {
      return info.password;
    }
    case kTlsRole: {
      return info.with_tls;
    }
    case kCleanSessionRole: {
      return info.clean_session;
    }
    case kDescriptionRole: {
      return info.description;
    }
    case kConnectionStateRole: {
      if (this->clients_.contains(info.id)) {
        auto* client = this->clients_.value(info.id);
        Q_ASSERT(client != nullptr);
        qDebug() << "client state:" << client->state();
        return static_cast<int>(client->state());
      } else {
        return static_cast<int>(ConnectionState::ConnectionDisconnected);
      }
    }
    default: {
      qWarning() << "Invalid role:" << role;
      return {};
    }
  }
}

QHash<int, QByteArray> ConnectionsModel::roleNames() const {
  // Map role index to qml property name.
  return {
      {kIdRole, kId},
      {kNameRole, kName},
      {kClientIdRole, kClientId},
      {kProtocolRole, kProtocol},
      {kHostRole, kHost},
      {kPortRole, kPort},
      {kQoSRole, kQoS},
      {kUsernameRole, kUsername},
      {kPasswordRole, kPassword},
      {kTlsRole, kTls},
      {kCleanSessionRole, kCleanSession},
      {kDescriptionRole, kDescription},
      {kConnectionStateRole, kConnectionState},
  };
}

void ConnectionsModel::addConnection(const ConnectConfig& config) {
  this->beginResetModel();
  this->configs_.append(config);
  this->endResetModel();

  // save to local file
  this->saveConnInfo();
}

void ConnectionsModel::saveConnInfo() {
  if (!dumpConnectConfigs(this->conn_file_, this->configs_)) {
    qWarning() << "Failed to save connection info to file:" << conn_file_;
  }
}

void ConnectionsModel::loadConnInfo() {
  if (!QFile::exists(this->conn_file_)) {
    return;
  }

  const bool ok = parseConnectConfigs(this->conn_file_, this->configs_);
  if (!ok) {
    qWarning() << "Failed to parse conn info file:" << this->conn_file_;
    return;
  }
}

QString ConnectionsModel::configId(int index) const {
  Q_ASSERT(index >= 0 && index < this->configs_.length());
  return this->configs_.at(index).id;
}

QVariantMap ConnectionsModel::config(const QString& config_id) const {
  for (const auto& config : this->configs_) {
    if (config.id == config_id) {
      return dumpConnectConfig(config).toVariantMap();
    }
  }
  qWarning() << "Failed to find config with id:" << config_id;
  return {};
}

MqttClient* ConnectionsModel::client(const QString& config_id) {
  if (this->clients_.contains(config_id)) {
    auto* client = this->clients_.value(config_id);
    Q_ASSERT(client != nullptr);
    return client;
  }

  for (const auto& config : this->configs_) {
    if (config.id == config_id) {
      auto* new_client = new MqttClient(this);
      connect(new_client, &MqttClient::stateChanged, [=]() {
        for (int index = 0; index < this->configs_.length(); ++index) {
          if (this->configs_.at(index).id == config_id) {
            emit this->dataChanged(this->index(index), this->index(index));
            return;
          }
        }
        qWarning() << "Failed to find config with name:" << config_id;
      });

      new_client->setConfig(config);
      this->clients_.insert(config_id, new_client);
      qDebug() << "Create new client:" << new_client;
      return new_client;
    }
  }

  qWarning() << "Invalid connection config id:" << config_id;
  return nullptr;
}

void ConnectionsModel::deleteRow(const QString& config_id) {
  Q_ASSERT(!config_id.isEmpty());
  if (this->clients_.contains(config_id)) {
    auto* client = this->clients_.take(config_id);
    client->deleteLater();
  }

  for (int index = 0; index < this->configs_.length(); ++index) {
    if (this->configs_.at(index).id == config_id) {
      this->beginRemoveRows(QModelIndex(), index, index);
      this->configs_.removeAt(index);
      this->endRemoveRows();
      this->saveConnInfo();
      break;
    }
  }
}

}  // namespace hebo