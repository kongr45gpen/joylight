#ifndef FIXTUREDIALOG_H
#define FIXTUREDIALOG_H

#include <iostream>

#include "fixture.h"

#include <QDialog>
#include <QListWidget>

namespace Ui {
class FixtureDialog;
}

class FixtureDialog : public QDialog
{
    Q_OBJECT

public:
    explicit FixtureDialog(QWidget *parent = nullptr);
    virtual ~FixtureDialog();


private:
    Ui::FixtureDialog *ui;

private slots:
    void openAddFixtureDialog();
    void addFixture(Fixture fixture);

};

#endif