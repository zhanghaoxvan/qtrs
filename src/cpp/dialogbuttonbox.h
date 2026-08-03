// src/cpp/dialogbuttonbox.h — QDialogButtonBox wrapper
#pragma once

#include <QtWidgets/QDialogButtonBox>
#include <QtWidgets/QPushButton>
#include <QtCore/QString>
#include <string>

inline QDialogButtonBox* QDialogButtonBox_new(QWidget* parent) {
    return new QDialogButtonBox(parent);
}

inline void QDialogButtonBox_setStandardButtons(QDialogButtonBox* button_box, int buttons) {
    button_box->setStandardButtons(static_cast<QDialogButtonBox::StandardButton>(buttons));
}

inline QPushButton* QDialogButtonBox_button(QDialogButtonBox* box, int button) {
    return box->button(static_cast<QDialogButtonBox::StandardButton>(button));
}

inline void QDialogButtonBox_delete(QDialogButtonBox* box) {
    delete box;
}