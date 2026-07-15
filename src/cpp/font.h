// src/cpp/font.h
#pragma once

#include <QFont>
#include <QString>
#include <string>
#include "rust/cxx.h"

// ============================================================
// Constructor
// ============================================================

/// Create a new QFont with default settings.
inline QFont* QFont_new() {
    return new QFont();
}

/// Create a new QFont with a specific family.
inline QFont* QFont_new_with_family(const std::string& family) {
    return new QFont(QString::fromStdString(family));
}

// ============================================================
// Setters
// ============================================================

/// Set the font family (e.g., "Arial", "WenQuanYi Micro Hei").
inline void QFont_setFamily(QFont* font, const std::string& family) {
    font->setFamily(QString::fromStdString(family));
}

/// Set the font size in points (1/72 inch).
inline void QFont_setPointSize(QFont* font, int size) {
    font->setPointSize(size);
}

/// Set the font size in pixels.
inline void QFont_setPixelSize(QFont* font, int size) {
    font->setPixelSize(size);
}

/// Enable/disable bold.
inline void QFont_setBold(QFont* font, bool bold) {
    font->setBold(bold);
}

/// Enable/disable italic.
inline void QFont_setItalic(QFont* font, bool italic) {
    font->setItalic(italic);
}

/// Enable/disable underline.
inline void QFont_setUnderline(QFont* font, bool underline) {
    font->setUnderline(underline);
}

/// Enable/disable strikethrough.
inline void QFont_setStrikeOut(QFont* font, bool strike) {
    font->setStrikeOut(strike);
}

/// Set the font weight (0-99).
///
/// Common values:
/// - 50 = Normal (default)
/// - 75 = Bold
/// - 25 = Light
/// - 87 = Black
inline void QFont_setWeight(QFont* font, int weight) {
    font->setWeight(static_cast<QFont::Weight>(weight));
}

// ============================================================
// Getters
// ============================================================

/// Get the font family name.
inline rust::String QFont_family(QFont* font) {
    return rust::String(font->family().toStdString());
}

/// Get the point size (returns -1 if not set).
inline int QFont_pointSize(QFont* font) {
    return font->pointSize();
}

/// Get the pixel size (returns -1 if not set).
inline int QFont_pixelSize(QFont* font) {
    return font->pixelSize();
}

/// Check if the font is bold.
inline bool QFont_bold(QFont* font) {
    return font->bold();
}

/// Check if the font is italic.
inline bool QFont_italic(QFont* font) {
    return font->italic();
}

/// Check if the font is underlined.
inline bool QFont_underline(QFont* font) {
    return font->underline();
}

/// Check if the font has strikethrough.
inline bool QFont_strikeOut(QFont* font) {
    return font->strikeOut();
}

/// Get the font weight (0-99).
inline int QFont_weight(QFont* font) {
    return font->weight();
}

// ============================================================
// Destructor
// ============================================================

/// Delete a QFont.
inline void QFont_delete(QFont* font) {
    delete font;
}