// Copyright (c) 2021 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import org.biofan.hebo 1.0
import "items" as Hebo

Item {
  id: root;
  property string connectId;

  property MqttClient client;

  FontLoader {
    id: iconFont;
    source: "fonts/element-icons.ttf";
  }

  ColumnLayout {
    anchors.fill: parent;

    RowLayout {
      width: parent.width;
      spacing: 12;

      Text {
        text: connectManager.config(root.connectId).name;
        font.pixelSize: 16;
        color: "#4d4d4d";
      }

      Item {
        height: 1;
        Layout.fillWidth: true;
      }

      IconButton {
        text: "\ue791";
        ToolTip.text: qsTr("Connect");
        visible: root.client.state !== MqttClient.ConnectionConnected;
        onClicked: {
          root.client.requestConnect();
        }
      }

      IconButton {
        text: "\ue71b";
        textColor: "red";
        ToolTip.text: qsTr("Disconnect");
        visible: root.client.state === MqttClient.ConnectionConnected;
        onClicked: {
          root.client.requestDisconnect();
        }
      }

      IconButton {
        text: "\ue78c";
        textColor: "gray";
        ToolTip.text: qsTr("Edit");
        onClicked: {
          const config = connectManager.config(root.connectId);
          if (!!config) {
            editConnectionDialog.connectionName = config.name;
            editConnectionDialog.clientId = config.clientId;
            editConnectionDialog.username = config.username;
            editConnectionDialog.password = config.password;
            editConnectionDialog.cleanSession = config.cleanSession;
            editConnectionDialog.open();
          } else {
            console.warn("Invalid config:", config, root.name);
          }
        }
      }

      IconButton {
        text: "\ue775";
        ToolTip.text: qsTr("New Window");
        onClicked: {
          console.log("popup new window");
        }
      }

      IconButton {
        text: "\ue794";
        ToolTip.text: qsTr("More");
        onClicked: {
          console.log("Show popup menu");
        }
      }
    }

    RowLayout {
      Pane {
        id: leftPane;
        width: 202;
        Layout.preferredWidth: 202;
        Layout.fillHeight: true;

        background: Rectangle {
          anchors.fill: parent;
          color: "#f1f1f1";
        }

        ColumnLayout {
          id: topicLayout;
          width: 188;
          Layout.preferredWidth: width;
          spacing: 12;

          Button {
            text: qsTr("New Subscription");
            Layout.preferredWidth: parent.width;
            onClicked: {
              if (root.client.state === MqttClient.ConnectionConnected) {
                newSubscriptionDialog.reset();
                newSubscriptionDialog.open();
              } else {
                console.warn("Invalid connection state");
              }
            }
          }

          ListView {
            id: subscriptionsList;
            Layout.fillHeight: true;
            spacing: 9;
            model: root.client.subscriptions;

            delegate: Rectangle {
              color: "#eaeaea";
              radius: 4;
              width: topicLayout.Layout.preferredWidth;
              height: topicLabel.height + 24;

              MouseArea {
                id: unsubscribeMA;
                anchors.fill: parent;
                hoverEnabled: true;
                onClicked: {
                  console.log("clicked, filter topic");
                }
              }

              Button {
                id: unsubscribeButton;
                visible: unsubscribeMA.containsMouse;
                anchors.right: parent.right;
                anchors.top: parent.top;
                text: "X";

                background: Rectangle {
                  color: "red";
                  opacity: 1;
                  width: 24;
                  height: 24;
                  radius: 12;
                }

                onClicked: {
                  // TODO(Shaohua): Check connection state.
                  root.client.requestUnsubscribe(model.topic);
                }
              }

              RowLayout {
                anchors.fill: parent;
                anchors.leftMargin: 8;
                anchors.rightMargin: 8;
                spacing: 8;

                Rectangle {
                  width: 16;
                  height: 16;
                  radius: 4;
                  color: model.color;
                }

                Text {
                  id: topicLabel;
                  text: model.topic;
                }

                Text {
                  Layout.alignment: Qt.AlignRight;
                  horizontalAlignment: Text.AlignRight;
                  color: "#313131";
                  text: "QoS " + model.qos;
                }
              }
            }
          }
        }
      }

      ColumnLayout {
        spacing: 0;

        ListView {
          id: messageStreamList;
          spacing: 12;
          clip: true;
          Layout.fillWidth: true;
          Layout.fillHeight: true;
          model: root.client.messages;

          onCountChanged: {
            this.positionViewAtEnd();
          }

          delegate: Column {
            anchors.right: model.isPublish ? messageStreamList.contentItem.right : undefined;

            Pane {
              width: parent.width;

              background: Rectangle {
                color: model.isPublish ? "#34c388" : "gray";
                radius: 14;
              }

              ColumnLayout {
                width: parent.width;
                spacing: 0;

                Label {
                  text: "Topic: " + model.topic;
                }

                Label {
                  text: "QoS " + model.qos;
                }

                Label {
                  text: model.payload;
                }
              }
            }

            Label {
              color: "gray";
              text: model.timestamp;
            }
          }
        }

        RowLayout {
          Hebo.FormLabel {
            text: "Payload:";
          }

          ComboBox{
            id: payloadTypeField;
            model: ["JSON", "Plaintext", "Base64", "Hex"];
          }

          Hebo.FormLabel {
            text: "QoS:";
          }

          ComboBox {
            id: qosTypeField;
            model: [0, 1, 2];
          }

          CheckBox {
            id: retainButtonField;
            text: "Retain";
          }
        }

        TextField {
          id: topicField;
          Layout.fillWidth: true;
          placeholderText: qsTr("Topic");
        }

        TextArea {
          id: payloadField;
          height: 148;
          Layout.fillWidth: true;
          Layout.preferredHeight: height;
          placeholderText: qsTr("Payload");
          background: Rectangle {
            anchors.fill: parent;
            color: "#fff";
            border.color: parent.focus ? "#0066ff" : "#c1c1c1";
            border.width: 2;
          }

          IconButton {
            anchors.right: parent.right;
            anchors.bottom: parent.bottom;
            anchors.rightMargin: 16;
            anchors.bottomMargin: 16;
            text: "\ue729";
            textColor: "#3a3a3a";
            ToolTip.text: qsTr("Send");
            onClicked: {
              if (root.client.state === MqttClient.ConnectionConnected) {
                root.client.requestPublish(topicField.text,
                  payloadField.text,
                  qosTypeField.currentIndex,
                  retainButtonField.checkState === Qt.Checked
                );
              } else {
                console.warn("Invalid mqtt connection state:", root.client.state);
              }
            }
          }
        }
      }
    }
  }

  Hebo.NewSubscriptionDialog {
    id: newSubscriptionDialog;

    onAccepted: {
      root.client.requestSubscribe(this.topic, this.qos, this.color);
    }
  }

  Hebo.EditConnectionDialog {
    id: editConnectionDialog;

    onAccepted: {
      console.log("TODO: Update connection info");
    }
  }

  Component.onCompleted: {
    this.client = connectManager.client(this.connectId);
  }

  component IconButton: Button {
    property color textColor;

    width: 32;
    height: 32;
    flat: true;
    Layout.preferredWidth: width;
    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter;
    ToolTip.visible: hovered;

    contentItem: Text {
      text: parent.text;
      color: parent.textColor;
      font.pixelSize: 24;
      font.family: iconFont.name;
      horizontalAlignment: Text.AlignHCenter;
      verticalAlignment: Text.AlignVCenter;
    }
  }
}