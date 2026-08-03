// src/cpp/wizard.h — QWizard
#pragma once

#include <QtWidgets/QWizard>
#include <QtWidgets/QWizardPage>
#include <QtCore/QString>
#include <string>
#include "rust/cxx.h"
#include "signal.h"

// QWizard
inline QWizard *QWizard_new(QWidget *parent) { return new QWizard(parent); }
inline void QWizard_delete(QWizard *w) { delete w; }
inline void QWizard_addPage(QWizard *w, QWizardPage *page) { w->addPage(page); }
inline void QWizard_setWindowTitle(QWizard *w, const std::string &title) {
    w->setWindowTitle(QString::fromStdString(title));
}
inline void QWizard_next(QWizard *w) { w->next(); }
inline void QWizard_back(QWizard *w) { w->back(); }
inline void QWizard_restart(QWizard *w) { w->restart(); }
inline int QWizard_currentId(QWizard *w) { return w->currentId(); }
inline QWidget *toQWidget_QWizard(QWizard *w) { return static_cast<QWidget *>(w); }

// QWizardPage
inline QWizardPage *QWizardPage_new(QWidget *parent) { return new QWizardPage(parent); }
inline void QWizardPage_delete(QWizardPage *p) { delete p; }
inline void QWizardPage_setTitle(QWizardPage *p, const std::string &title) {
    p->setTitle(QString::fromStdString(title));
}
inline void QWizardPage_setSubTitle(QWizardPage *p, const std::string &sub) {
    p->setSubTitle(QString::fromStdString(sub));
}
inline QWidget *toQWidget_QWizardPage(QWizardPage *p) {
    return static_cast<QWidget *>(p);
}
