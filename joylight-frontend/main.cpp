#include "mainwindow.h"

#include <iostream>

#include <QApplication>

#include "zmq.hpp"
#include "json.hpp"

int main(int argc, char *argv[])
{
    zmq::context_t context(1);

	zmq::socket_t requester(context, ZMQ_REQ);
	requester.connect("tcp://localhost:5555");
 
    std::cout << "Connection to ZeroMQ" << std::endl;
    requester.send(zmq::str_buffer("Give me your fixtures please"));
    zmq::message_t response;
    (void) requester.recv(response);
    
    std::cout << "Received reply " << " [" << response << "]" << std::endl;

    auto j = nlohmann::json::parse(response.to_string());

    std::cout << "Interesting light statistics:" << std::endl;
    std::cout << "  Number of lights: " << j["fixtures"].size() << std::endl;
    std::cout << "  Light names: ";
    for (auto& light : j["fixtures"]) {
        std::cout << light["name"] << ", ";
    }
    std::cout << "\n  Number of templates: " << j["templates"].size() << std::endl;
    std::cout << "  Template names: ";
    for (auto& template_ : j["templates"]) {
        std::cout << template_["name"] << ", ";
    }
    std::cout << std::endl;

    QApplication a(argc, argv);
    MainWindow w;
    w.setText(QString::fromStdString(j["templates"].dump(4)));
    w.show();
    return a.exec();
}
