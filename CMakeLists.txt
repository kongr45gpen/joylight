cmake_minimum_required(VERSION 3.16)
project(joylight LANGUAGES CXX)

find_package(Qt6 REQUIRED COMPONENTS Core Gui Widgets Network)

include_directories(${Qt6Core_INCLUDE_DIRS})
include_directories(${Qt6Gui_INCLUDE_DIRS})
include_directories(${Qt6Widgets_INCLUDE_DIRS})

add_definitions(${Qt6Core_DEFINITIONS})
add_definitions(${Qt6Gui_DEFINITIONS})
add_definitions(${Qt6Widgets_DEFINITIONS})

include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/inc
)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOUIC_SEARCH_PATHS ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/style/forms)

set(SOURCES
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/main.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/src/fixtureDialog.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/src/fixtureselect.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/src/mainwindow.cpp
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/src/socket.cpp
)

set(HEADERS
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/inc/config.h
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/inc/fixture.h
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/inc/fixtureDialog.h
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/inc/fixtureselect.h
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/inc/mainwindow.h
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/inc/socket.h
)

set(FORMS
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/style/forms/fixtureDialog.ui
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/style/forms/fixtureselect.ui
  ${CMAKE_CURRENT_SOURCE_DIR}/joylight-frontend/style/forms/mainwindow.ui  
  )

qt_wrap_cpp(MOC_SOURCES ${SOURCES})
qt_wrap_ui(UI_SOURCES ${FORMS})

add_executable(joylight ${SOURCES} ${HEADERS} ${FORMS})

target_link_libraries(joylight
    PRIVATE
        Qt::Core
        Qt::Gui
        Qt::Widgets
        Qt::Network
)
target_include_directories(joylight  PRIVATE ${CMAKE_CURRENT_BINARY_DIR})