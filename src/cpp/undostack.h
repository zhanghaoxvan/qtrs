// src/cpp/undostack.h — QUndoStack
#pragma once

#include <QUndoStack>
#include "signal.h"

inline QUndoStack *QUndoStack_new(QObject *parent) { return new QUndoStack(parent); }
inline void QUndoStack_delete(QUndoStack *s) { delete s; }
inline void QUndoStack_undo(QUndoStack *s) { s->undo(); }
inline void QUndoStack_redo(QUndoStack *s) { s->redo(); }
inline void QUndoStack_clear(QUndoStack *s) { s->clear(); }
inline bool QUndoStack_canUndo(QUndoStack *s) { return s->canUndo(); }
inline bool QUndoStack_canRedo(QUndoStack *s) { return s->canRedo(); }
inline int QUndoStack_count(QUndoStack *s) { return s->count(); }
