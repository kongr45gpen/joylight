#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include "fixture.h"

#include <QMainWindow>
#include <QMap>
#include <QUdpSocket>

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    virtual ~MainWindow();

    void setText(QString text);
private slots:
    void addFixture(Fixture fixture);
    void showFixturePatching();

private:
    Ui::MainWindow *ui;

    // Cached attributes to avoid back and forth with the backend 
    QMap<QString, Fixture> fixture_attribute;
};
#endif // MAINWINDOW_H
