// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_INTERNAL_NEW_SUBSCRIPTION_WINDOW_H_
#define HEBO_SRC_FRAMES_INTERNAL_NEW_SUBSCRIPTION_WINDOW_H_

#include <QComboBox>
#include <QMessageBox>
#include <QFrame>
#include <QLineEdit>

#include "frames/models/qos_model.h"
#include "widgets/color_line_edit.h"
#include "widgets/font_icon_button.h"

namespace hebo {

class NewSubscriptionWindow : public QFrame {
  Q_OBJECT
 public:
  explicit NewSubscriptionWindow(QWidget* parent = nullptr);

  [[nodiscard]] QString topic() const { return this->topic_edit_->text(); }
  [[nodiscard]] QoS qos() const {
    const auto index = this->qos_model_->index(this->qos_box_->currentIndex(), 0);
    return this->qos_model_->data(index, QoSModel::kIdRole).value<QoS>();
  }
  [[nodiscard]] QColor color() const { return this->color_edit_->color(); }
  [[nodiscard]] QString alias() const { return this->alias_edit_->text(); }

 signals:
  void confirmed();

 private:
  void initUi();
  void initSignals();

  QLineEdit* topic_edit_{nullptr};
  QComboBox* qos_box_{nullptr};
  QoSModel* qos_model_{nullptr};
  ColorLineEdit* color_edit_{nullptr};
  FontIconButton* refresh_color_button_{nullptr};
  QLineEdit* alias_edit_{nullptr};

  QPushButton* cancel_button_{nullptr};
  QPushButton* ok_button_{nullptr};
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_INTERNAL_NEW_SUBSCRIPTION_WINDOW_H_