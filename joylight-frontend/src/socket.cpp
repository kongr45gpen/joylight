#include <iostream>

#include "zmq.hpp"
#include "socket.h"
#include "config.h"

// TODO: Have a more proper initialisation of this
zmq::context_t context(1); 

// TODO: Should socket be global?
zmq::socket_t ConnectToSocket() {
    std::cout << "Connecting to socket..." << std::endl;

    zmq::socket_t requester(context, ZMQ_REQ);
	requester.connect(Address);

    std::cout << "Requester created. Open backend if not already up." << std::endl;

    return requester;
}
