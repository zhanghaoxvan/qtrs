import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Window 2.15

Window {
    id: root
    objectName: "loginWindow"
    width: 360
    height: 480
    title: "Login"
    visible: true

    Rectangle {
        anchors.fill: parent
        color: "#1a1a2e"
    }

    ColumnLayout {
        anchors.centerIn: parent
        spacing: 20
        width: 300

        Text {
            text: "Welcome"
            font.pixelSize: 32
            font.bold: true
            color: "white"
            horizontalAlignment: Text.AlignHCenter
            Layout.alignment: Qt.AlignHCenter
            Layout.bottomMargin: 20
        }

        TextField {
            id: usernameInput
            objectName: "usernameInput"
            Layout.fillWidth: true
            placeholderText: "Username"
            color: "white"
            font.pixelSize: 16
            background: Rectangle {
                color: "#16213e"
                radius: 8
                border.color: usernameInput.activeFocus ? "#0f3460" : "transparent"
            }
        }

        TextField {
            id: passwordInput
            objectName: "passwordInput"
            Layout.fillWidth: true
            placeholderText: "Password"
            echoMode: TextInput.Password
            color: "white"
            font.pixelSize: 16
            background: Rectangle {
                color: "#16213e"
                radius: 8
                border.color: passwordInput.activeFocus ? "#0f3460" : "transparent"
            }
        }

        Button {
            id: loginButton
            Layout.fillWidth: true
            text: "Login"
            font.pixelSize: 18
            font.bold: true

            background: Rectangle {
                color: "#0f3460"
                radius: 8
            }
            contentItem: Text {
                text: loginButton.text
                color: "white"
                font: loginButton.font
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
            }

            onClicked: {
                backend.login(usernameInput.text, passwordInput.text)
            }
        }

        Text {
            id: statusText
            objectName: "statusText"
            text: ""
            color: "#ff6b6b"
            font.pixelSize: 14
            horizontalAlignment: Text.AlignHCenter
            Layout.alignment: Qt.AlignHCenter
            visible: text !== ""
        }
    }

    Connections {
        target: backend
        function onLoginResult(success, message) {
            if (success) {
                statusText.color = "#51cf66"
                statusText.text = " OK  " + message
                loginTimer.start()
            } else {
                statusText.color = "#ff6b6b"
                statusText.text = "FAIL " + message
            }
        }
    }

    Timer {
        id: loginTimer
        interval: 1000
        onTriggered: root.close()
    }
}
