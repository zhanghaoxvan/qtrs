// src/cpp/textbrowser.h — QTextBrowser
#pragma once

#include <QtCore/QObject>
#include <QtCore/QString>
#include <QtCore/QUrl>
#include <QtCore/QStringList>
#include <QtWidgets/QTextBrowser>
#include <string>
#include <vector>
#include "rust/cxx.h"

#include "rust/cxx.h"
#include "signal.h"

inline QTextBrowser *QTextBrowser_new(QWidget *parent) {
    return new QTextBrowser(parent);
}

inline void QTextBrowser_delete(QTextBrowser *w) { delete w; }

inline void QTextBrowser_setHtml(QTextBrowser *w, const std::string &html) {
    w->setHtml(QString::fromStdString(html));
}

inline void QTextBrowser_setPlainText(QTextBrowser *w, const std::string &text) {
    w->setPlainText(QString::fromStdString(text));
}

inline rust::String QTextBrowser_plainText(QTextBrowser *w) {
    return w->toPlainText().toStdString();
}

inline rust::String QTextBrowser_toHtml(QTextBrowser *w) {
    return w->toHtml().toStdString();
}

inline void QTextBrowser_setOpenExternalLinks(QTextBrowser *w, bool open) {
    w->setOpenExternalLinks(open);
}

inline void QTextBrowser_setOpenLinks(QTextBrowser *w, bool open) {
    w->setOpenLinks(open);
}

inline void QTextBrowser_setSource(QTextBrowser *w, const std::string &url) {
    w->setSource(QUrl(QString::fromStdString(url)));
}

inline rust::String QTextBrowser_source(QTextBrowser *w) {
    return w->source().toString().toStdString();
}

inline void QTextBrowser_clear(QTextBrowser *w) {
    w->clear();
}

inline void QTextBrowser_append(QTextBrowser *w, const std::string &text) {
    w->append(QString::fromStdString(text));
}

inline void QTextBrowser_setSearchPaths(QTextBrowser *w, rust::Vec<rust::String> paths) {
    QStringList list;
    for (const auto &p : paths) {
        list.append(QString::fromStdString(std::string(p)));
    }
    w->setSearchPaths(list);
}

inline void QTextBrowser_onAnchorClicked(QTextBrowser *w, uint64_t ctx) {
    QObject::connect(w, &QTextBrowser::anchorClicked, [ctx](const QUrl &url) {
        if (g_hasStringTrampoline) {
            rust::String s = rust::String(url.toString().toStdString());
            g_stringTrampoline(ctx, std::move(s));
        }
    });
}

inline void QTextBrowser_onTextChanged(QTextBrowser *w, uint64_t ctx) {
    QObject::connect(w, &QTextBrowser::textChanged, [ctx]() {
        if (g_hasVoidTrampoline) g_voidTrampoline(ctx);
    });
}

inline QWidget *toQWidget_QTextBrowser(QTextBrowser *w) {
    return static_cast<QWidget *>(w);
}
