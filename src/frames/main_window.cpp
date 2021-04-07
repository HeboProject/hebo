// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/main_window.h"

#include <QDebug>

namespace hebo {

MainWindow::MainWindow(QWidget* parent) : QWidget(parent) {
  this->initUi();
  this->initSignals();
  this->left_panel_->setActiveButton(LeftPanel::kConnectionsButton);
}

void MainWindow::initUi() {
  auto* main_layout = new QHBoxLayout();
  main_layout->setSpacing(0);
  main_layout->setContentsMargins(0, 0, 0, 0);
  this->setLayout(main_layout);

  this->left_panel_ = new LeftPanel();
  main_layout->addWidget(this->left_panel_);

  this->stacked_layout_ = new QStackedLayout();
  main_layout->addLayout(this->stacked_layout_);

  this->connections_window_ = new ConnectionsWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kConnectionsButton, this->connections_window_);

  this->new_connection_window_ = new NewConnectionWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kNewConnectionButton, this->new_connection_window_);

  this->benchmark_window_ = new BenchmarkWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kBenchmarkButton, this->benchmark_window_);

  this->bag_window_ = new BagWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kBagButton, this->bag_window_);

  this->log_window_ = new LogWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kLogButton, this->log_window_);

  this->about_window_ = new AboutWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kAboutButton, this->about_window_);

  this->settings_window_ = new SettingsWindow();
  this->stacked_layout_->insertWidget(LeftPanel::kSettingsButton, this->settings_window_);
}

void MainWindow::initSignals() {
  connect(this->left_panel_, &LeftPanel::activeChanged,
          this, &MainWindow::switchWindowBydId);
  connect(this->new_connection_window_, &NewConnectionWindow::newConnectionAdded, [=](const QString& client_id) {
    this->left_panel_->setActiveButton(LeftPanel::kConnectionsButton);
    this->connections_window_->connectClient(client_id);
  });
}

void MainWindow::setConnectionsModel(ConnectionsModel* model) {
  this->connections_window_->setConnectionsModel(model);
  this->new_connection_window_->setConnectionsModel(model);
}

void MainWindow::switchWindowBydId(LeftPanel::ButtonId id) {
  this->stacked_layout_->setCurrentIndex(id);
  auto* widget = this->stacked_layout_->widget(id);
  if (widget != nullptr) {
    this->setWindowTitle(widget->windowTitle());
  } else {
    qCritical() << "widget is null, id:" << id;
  }
}

}  // namespace hebo