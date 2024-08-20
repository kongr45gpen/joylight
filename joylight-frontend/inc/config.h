#ifndef CONFIG_H
#define CONFIG_H

#include <QUdpSocket>
#include <QString>

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

inline const std::string Address = "tcp://localhost:5555";

#endif // CONFIG_H
