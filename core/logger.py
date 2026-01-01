#!/usr/bin/env python3

import logging
import os
from datetime import datetime
from pathlib import Path


def setup_dated_logger():

    now = datetime.now()
    year = now.strftime("%Y")
    month = now.strftime("%B")
    day = now.strftime("%d")

    log_dir = Path("logs") / year / month
    log_dir.mkdir(parents=True, exist_ok=True)

    log_file = log_dir / f"{day}.log"

    logger = logging.getLogger("DirtyChess")
    logger.setLevel(logging.DEBUG)

    file_handler = logging.FileHandler(log_file)
    formatter = logging.Formatter(
        "%(asctime)s - %(name)s - %(levelname)s - %(message)s"
    )
    file_handler.setFormatter(formatter)

    if not logger.handlers:
        logger.addHandler(file_handler)

    return logger


logger = setup_dated_logger()
