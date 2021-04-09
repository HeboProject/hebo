// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "frames/internal/new_subscription_window.h"

#include <QFormLayout>
#include <QLabel>

#include "resources/fonts/fonts.h"

namespace hebo {

NewSubscriptionWindow::NewSubscriptionWindow(QWidget* parent) : QFrame(parent) {
  this->initUi();
  this->initSignals();
}

void NewSubscriptionWindow::initUi() {
  this->setWindowTitle(tr("New Subscription"));

  auto* main_layout = new QVBoxLayout();
  this->setLayout(main_layout);

  auto* form_layout = new QFormLayout();
  main_layout->addLayout(form_layout);

  this->topic_edit_ = new QLineEdit();
  form_layout->addRow(new QLabel(tr("Topic")), this->topic_edit_);

  this->qos_box_ = new QComboBox();
  this->qos_model_ = new QoSModel(this);
  this->qos_box_->setModel(this->qos_model_);
  form_layout->addRow(new QLabel(tr("QoS")), this->qos_box_);

  auto* color_layout = new QHBoxLayout();
  this->color_edit_ = new ColorLineEdit();
  color_layout->addWidget(this->color_edit_);
  this->refresh_color_button_ = new FontIconButton(kFontElIconRefresh);
  color_layout->addWidget(this->refresh_color_button_);
  form_layout->addRow(new QLabel(tr("Color")), color_layout);

  this->alias_edit_ = new QLineEdit();
  form_layout->addRow(new QLabel(tr("Alias")), this->alias_edit_);

  auto* buttons_layout = new QHBoxLayout();
  main_layout->addSpacing(12);
  main_layout->addLayout(buttons_layout);
  this->cancel_button_ = new QPushButton(tr("Cancel"));
  this->ok_button_ = new QPushButton(tr("Ok"));
  buttons_layout->addWidget(this->cancel_button_);
  buttons_layout->addWidget(this->ok_button_);
}

void NewSubscriptionWindow::initSignals() {
  connect(this->cancel_button_, &QPushButton::clicked,
          this, &NewSubscriptionWindow::hide);
  connect(this->ok_button_, &QPushButton::clicked,
          this, &NewSubscriptionWindow::confirmed);
}

}  // namespace hebo