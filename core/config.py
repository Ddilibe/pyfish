#!/usr/bin/env python3

from pydantic import BaseModel, Field
from typing import Tuple
import json


class DisplayConfig(BaseModel):
    width: int = 1000
    height: int = 800
    fps: int = 60
    title: str = "Chess Master 2026"


class BoardConfig(BaseModel):
    size: int = 480
    square_size: int = 60
    offset: Tuple[int, int] = (100, 50)
    light_color: str = "#ebecd0"
    dark_color: str = "#779556"


class AssetConfig(BaseModel):
    theme_path: str = "theme.json"
    font_path: str = "assets/fonts/Moralana DEMO.otf"
    logo_path: str = "assets/logo.png"
    music_path: str = "assets/sounds/Limujii - November (freetouse.com).mp3"


class AppConfig(BaseModel):
    display: DisplayConfig
    board: BoardConfig
    assets: AssetConfig


def load_config(file_path: str = "config.json") -> AppConfig:
    try:
        with open(file_path, "r") as f:
            data = json.load(f)
            return AppConfig(**data)
    except FileNotFoundError:
        # Fallback to defaults if file is missing
        return AppConfig(
            display=DisplayConfig(), board=BoardConfig(), assets=AssetConfig()
        )
    except Exception as e:
        print(f"Configuration Error: {e}")
        raise
