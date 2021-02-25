// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

#include "controllers/shell.h"

#include <ConsoleAppender.h>
#include <Logger.h>
#include <QGuiApplication>

#include "controllers/main_controller.h"
#include "mqtt/contact_model.h"

namespace hebo {

int runShell(int argc, char** argv) {
  QGuiApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
  QGuiApplication::setApplicationDisplayName("Hebo UI");
  QGuiApplication::setApplicationName("HeboUi");
  QGuiApplication::setApplicationVersion("0.1.0");
  QGuiApplication::setDesktopFileName("hebo-ui");
  QGuiApplication::setOrganizationDomain("biofan.org");
  QGuiApplication::setOrganizationName("HeboUi");
//  QGuiApplication::setWindowIcon(QIcon(kHeboUiIcon));

  QGuiApplication application(argc, argv);
  registerComponents();
  cuteLogger->registerAppender(new ConsoleAppender());

  QScopedPointer<MainController> controller(new MainController());
  controller->showMainWindow();

  return QGuiApplication::exec();
}

void registerComponents() {
  constexpr const char* kComponentUri = "HeboCpp";
  constexpr const char* kComponentName = "HeboCpp";
  constexpr int kVersionMajor = 1;
  constexpr int kVersionMinor = 0;
  qmlRegisterUncreatableMetaObject(hebo::staticMetaObject,
                                   kComponentUri,
                                   kVersionMajor, kVersionMinor,
                                   kComponentName,
                                   "Access to enums & flags only");
}

}  // namespace hebo
