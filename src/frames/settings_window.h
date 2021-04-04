// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#ifndef HEBO_SRC_FRAMES_SETTINGS_WINDOW_H_
#define HEBO_SRC_FRAMES_SETTINGS_WINDOW_H_

#include <QWidget>

namespace hebo {

class SettingsWindow : public QWidget {
  Q_OBJECT
 public:
  explicit SettingsWindow(QWidget* parent = nullptr);
};

}  // namespace hebo

#endif  // HEBO_SRC_FRAMES_SETTINGS_WINDOW_H_
