// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_MQTT_INTERNAL_CLIENT_H_
#define HEBO_SRC_MQTT_INTERNAL_CLIENT_H_

#include <QObject>
#include <QTimer>

#include "formats/connect_config.h"
#include "mqtt/message_stream_model.h"

namespace hebo {

struct MqttClientPrivate;

class InternalClient : public QObject {
  Q_OBJECT
 public:
  explicit InternalClient(QObject* parent = nullptr);
  ~InternalClient() override;

 signals:
  void requestConnect(const ConnectConfig& config);
  void requestDisconnect();
  void requestSubscribe(const QString& topic, QoS qos);
  void requestUnsubscribe(const QString& topic);
  void requestPublish(const QString& topic, const QByteArray& payload, QoS qos, bool retain);

  void stateChanged(ConnectionState state);

  void messagesReceived(const MqttMessages& messages);

 protected:
  void timerEvent(QTimerEvent* event) override;

 private slots:
  void doConnect(const ConnectConfig& config);
  void doDisconnect();
  void doSubscribe(const QString& topic, QoS qos);
  void doUnsubscribe(const QString& topic);
  void doPublish(const QString& topic, const QByteArray& payload, QoS qos, bool retain);

  void onQueuedMessagesTimeout();

 private:
  void initSignals();
  MqttClientPrivate* p_;
  int timer_id_{-1};

  QTimer* queued_messages_timer_{nullptr};
  MqttMessages queued_messages_{};
};

}  // namespace hebo

#endif  // HEBO_SRC_MQTT_INTERNAL_CLIENT_H_
