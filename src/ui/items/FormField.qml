// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15

TextField {
  id: root;
  property bool isValid: false;
  property bool textFirstModified: false;

  selectByMouse: true;

  background: Rectangle {
    implicitWidth: 200;
    implicitHeight: 40;
    color: "transparent";

    border.width: root.focus ? 2 : 1;
    border.color: {
      if (root.textFirstModified && !root.isValid) {
        return "red";
      } else if (root.focus) {
        return "#0066ff";
      } else {
       return "#bdbdbd";
     }
    }
  }

  function runValidate() {
    this.textFirstModified = true;
  }

  function reset() {
    this.textFirstModified = false;
    this.text = "";
  }

  onEditingFinished: this.runValidate();
}
