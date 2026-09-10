#!/usr/bin/env python3

import ctypes
import json
import os
import sys


TITLE = b"BitBook accounts"
MAX_WINDOWS = 4096
CLIENT_MESSAGE = 33
IS_VIEWABLE = 2


class HelperError(Exception):
    pass


class XWindowAttributes(ctypes.Structure):
    _fields_ = [
        ("x", ctypes.c_int),
        ("y", ctypes.c_int),
        ("width", ctypes.c_int),
        ("height", ctypes.c_int),
        ("border_width", ctypes.c_int),
        ("depth", ctypes.c_int),
        ("visual", ctypes.c_void_p),
        ("root", ctypes.c_ulong),
        ("klass", ctypes.c_int),
        ("bit_gravity", ctypes.c_int),
        ("win_gravity", ctypes.c_int),
        ("backing_store", ctypes.c_int),
        ("backing_planes", ctypes.c_ulong),
        ("backing_pixel", ctypes.c_ulong),
        ("save_under", ctypes.c_int),
        ("colormap", ctypes.c_ulong),
        ("map_installed", ctypes.c_int),
        ("map_state", ctypes.c_int),
        ("all_event_masks", ctypes.c_long),
        ("your_event_mask", ctypes.c_long),
        ("do_not_propagate_mask", ctypes.c_long),
        ("override_redirect", ctypes.c_int),
        ("screen", ctypes.c_void_p),
    ]


class ClientMessageData(ctypes.Union):
    _fields_ = [
        ("b", ctypes.c_char * 20),
        ("s", ctypes.c_short * 10),
        ("l", ctypes.c_long * 5),
    ]


class XClientMessageEvent(ctypes.Structure):
    _fields_ = [
        ("type", ctypes.c_int),
        ("serial", ctypes.c_ulong),
        ("send_event", ctypes.c_int),
        ("display", ctypes.c_void_p),
        ("window", ctypes.c_ulong),
        ("message_type", ctypes.c_ulong),
        ("format", ctypes.c_int),
        ("data", ClientMessageData),
    ]


class XEvent(ctypes.Union):
    _fields_ = [
        ("xclient", XClientMessageEvent),
        ("pad", ctypes.c_long * 24),
    ]


def load_x11():
    try:
        lib = ctypes.CDLL("libX11.so.6")
    except OSError as error:
        raise HelperError("X11 library unavailable") from error

    lib.XOpenDisplay.argtypes = [ctypes.c_char_p]
    lib.XOpenDisplay.restype = ctypes.c_void_p
    lib.XDefaultRootWindow.argtypes = [ctypes.c_void_p]
    lib.XDefaultRootWindow.restype = ctypes.c_ulong
    lib.XQueryTree.argtypes = [
        ctypes.c_void_p,
        ctypes.c_ulong,
        ctypes.POINTER(ctypes.c_ulong),
        ctypes.POINTER(ctypes.c_ulong),
        ctypes.POINTER(ctypes.POINTER(ctypes.c_ulong)),
        ctypes.POINTER(ctypes.c_uint),
    ]
    lib.XQueryTree.restype = ctypes.c_int
    lib.XFetchName.argtypes = [
        ctypes.c_void_p,
        ctypes.c_ulong,
        ctypes.POINTER(ctypes.c_char_p),
    ]
    lib.XFetchName.restype = ctypes.c_int
    lib.XGetWindowAttributes.argtypes = [
        ctypes.c_void_p,
        ctypes.c_ulong,
        ctypes.POINTER(XWindowAttributes),
    ]
    lib.XGetWindowAttributes.restype = ctypes.c_int
    lib.XInternAtom.argtypes = [ctypes.c_void_p, ctypes.c_char_p, ctypes.c_int]
    lib.XInternAtom.restype = ctypes.c_ulong
    lib.XSendEvent.argtypes = [
        ctypes.c_void_p,
        ctypes.c_ulong,
        ctypes.c_int,
        ctypes.c_long,
        ctypes.POINTER(XEvent),
    ]
    lib.XSendEvent.restype = ctypes.c_int
    lib.XFlush.argtypes = [ctypes.c_void_p]
    lib.XFlush.restype = ctypes.c_int
    lib.XSync.argtypes = [ctypes.c_void_p, ctypes.c_int]
    lib.XSync.restype = ctypes.c_int
    lib.XFree.argtypes = [ctypes.c_void_p]
    lib.XFree.restype = ctypes.c_int
    lib.XCloseDisplay.argtypes = [ctypes.c_void_p]
    lib.XCloseDisplay.restype = ctypes.c_int
    lib.XSetErrorHandler.argtypes = [ctypes.c_void_p]
    lib.XSetErrorHandler.restype = ctypes.c_void_p
    return lib


def matching_windows(lib, display):
    root = lib.XDefaultRootWindow(display)
    if root == 0:
        raise HelperError("X11 root unavailable")

    pending = [root]
    seen = set()
    matches = []
    while pending:
        window = pending.pop()
        if window in seen:
            continue
        seen.add(window)
        if len(seen) > MAX_WINDOWS:
            raise HelperError("X11 window limit exceeded")

        root_return = ctypes.c_ulong()
        parent_return = ctypes.c_ulong()
        children = ctypes.POINTER(ctypes.c_ulong)()
        child_count = ctypes.c_uint()
        try:
            status = lib.XQueryTree(
                display,
                window,
                ctypes.byref(root_return),
                ctypes.byref(parent_return),
                ctypes.byref(children),
                ctypes.byref(child_count),
            )
            if status == 0:
                raise HelperError("XQueryTree failed")
            if len(seen) + len(pending) + child_count.value > MAX_WINDOWS:
                raise HelperError("X11 window limit exceeded")
            for index in range(child_count.value):
                pending.append(int(children[index]))
        finally:
            if children:
                lib.XFree(ctypes.cast(children, ctypes.c_void_p))

        name = ctypes.c_char_p()
        try:
            named = lib.XFetchName(display, window, ctypes.byref(name))
            if named == 0 or not name or ctypes.string_at(name) != TITLE:
                continue
            attributes = XWindowAttributes()
            if lib.XGetWindowAttributes(display, window, ctypes.byref(attributes)) == 0:
                raise HelperError("XGetWindowAttributes failed")
            matches.append({"id": int(window), "map_state": int(attributes.map_state)})
        finally:
            if name:
                lib.XFree(ctypes.cast(name, ctypes.c_void_p))

    matches.sort(key=lambda value: value["id"])
    return matches


def parse_window_id(value):
    if not value or not value.isascii() or not value.isdecimal():
        raise HelperError("invalid window id")
    window = int(value, 10)
    maximum = min((1 << (ctypes.sizeof(ctypes.c_ulong) * 8)) - 1, 0xFFFFFFFF)
    if window <= 0 or window > maximum:
        raise HelperError("invalid window id")
    return window


def send_close(lib, display, window, windows):
    selected = [entry for entry in windows if entry["id"] == window]
    if len(selected) != 1 or selected[0]["map_state"] != IS_VIEWABLE:
        raise HelperError("window is not a visible matching window")

    protocols = lib.XInternAtom(display, b"WM_PROTOCOLS", 0)
    delete_window = lib.XInternAtom(display, b"WM_DELETE_WINDOW", 0)
    if protocols == 0 or delete_window == 0:
        raise HelperError("window protocol unavailable")

    event = XEvent()
    event.xclient.type = CLIENT_MESSAGE
    event.xclient.serial = 0
    event.xclient.send_event = 1
    event.xclient.display = display
    event.xclient.window = window
    event.xclient.message_type = protocols
    event.xclient.format = 32
    event.xclient.data.l[0] = delete_window
    event.xclient.data.l[1] = 0
    if lib.XSendEvent(display, window, 0, 0, ctypes.byref(event)) == 0:
        raise HelperError("XSendEvent failed")
    lib.XFlush(display)


def run():
    if len(sys.argv) == 2 and sys.argv[1] == "inspect":
        mode = "inspect"
        window = None
    elif len(sys.argv) == 3 and sys.argv[1] == "close":
        mode = "close"
        window = parse_window_id(sys.argv[2])
    else:
        raise HelperError("invalid arguments")

    display_name = os.environ.get("DISPLAY")
    if not display_name:
        raise HelperError("DISPLAY is unavailable")
    lib = load_x11()
    display = lib.XOpenDisplay(display_name.encode("utf-8"))
    if not display:
        raise HelperError("XOpenDisplay failed")

    errors = []
    error_handler_type = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.c_void_p, ctypes.c_void_p)

    @error_handler_type
    def record_x_error(_display, _event):
        errors.append(True)
        return 0

    previous_handler = lib.XSetErrorHandler(ctypes.cast(record_x_error, ctypes.c_void_p))
    try:
        windows = matching_windows(lib, display)
        if mode == "close":
            send_close(lib, display, window, windows)
        lib.XSync(display, 0)
        if errors:
            raise HelperError("X11 protocol error")
        if mode == "inspect":
            result = {"windows": windows}
        else:
            result = {"closed": window}
        sys.stdout.write(json.dumps(result, separators=(",", ":"), sort_keys=True) + "\n")
    finally:
        lib.XSetErrorHandler(previous_handler)
        lib.XCloseDisplay(display)


if __name__ == "__main__":
    try:
        run()
    except Exception:
        sys.stderr.write("x11 helper failed\n")
        sys.exit(1)
