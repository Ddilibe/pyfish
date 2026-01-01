#!/usr/bin/env python3

from enum import StrEnum


class ChessStateArgs(StrEnum):

    MENU = "MENU"
    PLAY = "PLAY"
    PAUSE = "PAUSE"
    QUIT = "QUIT"
    SETTINGS = "SETTINGS"
    HELP = "HELP"
    WELCOME = "WELCOME"


class State:

    __current_state = ChessStateArgs.WELCOME

    def __init__(self):
        pass

    @property
    def current_state(self):
        return self.__current_state

    @current_state.setter
    def current_state(self, new_state: ChessStateArgs):
        if not isinstance(new_state, ChessStateArgs):
            raise Exception(f"Invalid state substitution got {type(new_state)}")
        self.__current_state = new_state


customstate = State()
