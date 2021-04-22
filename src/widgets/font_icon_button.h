// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_WIDGETS_FONT_ICON_BUTTON_H_
#define HEBO_SRC_WIDGETS_FONT_ICON_BUTTON_H_

#include <QPushButton>

namespace hebo {

class FontIconButton : public QPushButton {
  Q_OBJECT
 public:
  explicit FontIconButton(const QString& text, QWidget* parent = nullptr);
  ~FontIconButton() override = default;
};

}  // namespace hebo

#endif  // HEBO_SRC_WIDGETS_FONT_ICON_BUTTON_H_
