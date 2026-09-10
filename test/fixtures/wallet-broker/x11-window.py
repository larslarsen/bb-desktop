#!/usr/bin/env python3

import base64
import ctypes
import json
import os
import struct
import sys
import zlib


TITLE = b"BitBook accounts"
MAX_WINDOWS = 4096
CLIENT_MESSAGE = 33
IS_VIEWABLE = 2
ZPIXMAP = 2
CAPTURE_MARGIN = 8
MAX_DIMENSION = 2048
XA_CARDINAL = 6
MAX_PID = (1 << 31) - 1


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


class Visual(ctypes.Structure):
    _fields_ = [
        ("ext_data", ctypes.c_void_p),
        ("visualid", ctypes.c_ulong),
        ("klass", ctypes.c_int),
        ("red_mask", ctypes.c_ulong),
        ("green_mask", ctypes.c_ulong),
        ("blue_mask", ctypes.c_ulong),
        ("bits_per_rgb", ctypes.c_int),
        ("map_entries", ctypes.c_int),
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
    lib.XGetWindowProperty.argtypes = [
        ctypes.c_void_p,
        ctypes.c_ulong,
        ctypes.c_ulong,
        ctypes.c_long,
        ctypes.c_long,
        ctypes.c_int,
        ctypes.c_ulong,
        ctypes.POINTER(ctypes.c_ulong),
        ctypes.POINTER(ctypes.c_int),
        ctypes.POINTER(ctypes.c_ulong),
        ctypes.POINTER(ctypes.c_ulong),
        ctypes.POINTER(ctypes.POINTER(ctypes.c_ubyte)),
    ]
    lib.XGetWindowProperty.restype = ctypes.c_int
    lib.XGetImage.argtypes = [
        ctypes.c_void_p,
        ctypes.c_ulong,
        ctypes.c_int,
        ctypes.c_int,
        ctypes.c_uint,
        ctypes.c_uint,
        ctypes.c_ulong,
        ctypes.c_int,
    ]
    lib.XGetImage.restype = ctypes.c_void_p
    lib.XGetPixel.argtypes = [ctypes.c_void_p, ctypes.c_int, ctypes.c_int]
    lib.XGetPixel.restype = ctypes.c_ulong
    lib.XDestroyImage.argtypes = [ctypes.c_void_p]
    lib.XDestroyImage.restype = ctypes.c_int
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


def window_pid(lib, display, window, pid_atom):
    actual_type = ctypes.c_ulong()
    actual_format = ctypes.c_int()
    item_count = ctypes.c_ulong()
    trailing_bytes = ctypes.c_ulong()
    storage = ctypes.POINTER(ctypes.c_ubyte)()
    try:
        status = lib.XGetWindowProperty(
            display,
            window,
            pid_atom,
            0,
            1,
            0,
            XA_CARDINAL,
            ctypes.byref(actual_type),
            ctypes.byref(actual_format),
            ctypes.byref(item_count),
            ctypes.byref(trailing_bytes),
            ctypes.byref(storage),
        )
        if status != 0:
            raise HelperError("XGetWindowProperty failed")
        if (actual_type.value != XA_CARDINAL or actual_format.value != 32 or
                item_count.value != 1 or trailing_bytes.value != 0 or not storage):
            return None
        return int(ctypes.cast(storage, ctypes.POINTER(ctypes.c_ulong))[0])
    finally:
        if storage:
            lib.XFree(ctypes.cast(storage, ctypes.c_void_p))


def matching_windows(lib, display, pid):
    root = lib.XDefaultRootWindow(display)
    if root == 0:
        raise HelperError("X11 root unavailable")
    pid_atom = lib.XInternAtom(display, b"_NET_WM_PID", 0)
    if pid_atom == 0:
        raise HelperError("X11 PID property unavailable")

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
            if window_pid(lib, display, window, pid_atom) != pid:
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


def parse_pid(value):
    if not value or not value.isascii() or not value.isdecimal():
        raise HelperError("invalid pid")
    pid = int(value, 10)
    if pid <= 0 or pid > MAX_PID:
        raise HelperError("invalid pid")
    return pid


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


def color_component(pixel, mask):
    if mask == 0:
        raise HelperError("unsupported X11 visual")
    shift = (mask & -mask).bit_length() - 1
    maximum = mask >> shift
    return (((pixel & mask) >> shift) * 255 + maximum // 2) // maximum


def png_chunk(kind, payload):
    content = kind + payload
    return struct.pack(">I", len(payload)) + content + struct.pack(">I", zlib.crc32(content))


def encode_png(width, height, pixels):
    rows = bytearray()
    stride = width * 3
    for y in range(height):
        rows.append(0)
        start = y * stride
        rows.extend(pixels[start:start + stride])
    return b"".join([
        b"\x89PNG\r\n\x1a\n",
        png_chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0)),
        png_chunk(b"IDAT", zlib.compress(bytes(rows), 9)),
        png_chunk(b"IEND", b""),
    ])


def capture_window(lib, display, window, windows):
    selected = [entry for entry in windows if entry["id"] == window]
    if len(selected) != 1 or selected[0]["map_state"] != IS_VIEWABLE:
        raise HelperError("window is not a visible matching window")

    attributes = XWindowAttributes()
    if lib.XGetWindowAttributes(display, window, ctypes.byref(attributes)) == 0:
        raise HelperError("XGetWindowAttributes failed")
    width = int(attributes.width)
    height = int(attributes.height)
    if (width <= CAPTURE_MARGIN * 2 or height <= CAPTURE_MARGIN * 2 or
            width > MAX_DIMENSION or height > MAX_DIMENSION):
        raise HelperError("invalid capture dimensions")
    if not attributes.visual:
        raise HelperError("X11 visual unavailable")
    visual = ctypes.cast(attributes.visual, ctypes.POINTER(Visual)).contents
    masks = (int(visual.red_mask), int(visual.green_mask), int(visual.blue_mask))
    if any(mask == 0 for mask in masks):
        raise HelperError("unsupported X11 visual")

    all_planes = (1 << (ctypes.sizeof(ctypes.c_ulong) * 8)) - 1
    image = lib.XGetImage(display, window, 0, 0, width, height, all_planes, ZPIXMAP)
    if not image:
        raise HelperError("XGetImage failed")
    pixels = bytearray(width * height * 3)
    colors = {}
    try:
        offset = 0
        for y in range(height):
            for x in range(width):
                pixel = int(lib.XGetPixel(image, x, y))
                red = color_component(pixel, masks[0])
                green = color_component(pixel, masks[1])
                blue = color_component(pixel, masks[2])
                pixels[offset:offset + 3] = bytes((red, green, blue))
                offset += 3
                if (CAPTURE_MARGIN <= x < width - CAPTURE_MARGIN and
                        CAPTURE_MARGIN <= y < height - CAPTURE_MARGIN):
                    color = (red << 16) | (green << 8) | blue
                    colors[color] = colors.get(color, 0) + 1
    finally:
        if lib.XDestroyImage(image) == 0:
            raise HelperError("XDestroyImage failed")

    sample_count = (width - CAPTURE_MARGIN * 2) * (height - CAPTURE_MARGIN * 2)
    dominant = max(colors.values())
    png = encode_png(width, height, pixels)
    return {
        "width": width,
        "height": height,
        "color_count": len(colors),
        "non_dominant_pixels": sample_count - dominant,
        "png_base64": base64.b64encode(png).decode("ascii"),
    }


def run():
    if len(sys.argv) == 3 and sys.argv[1] == "inspect":
        mode = "inspect"
        window = None
        pid = parse_pid(sys.argv[2])
    elif len(sys.argv) == 4 and sys.argv[1] == "close":
        mode = "close"
        window = parse_window_id(sys.argv[2])
        pid = parse_pid(sys.argv[3])
    elif len(sys.argv) == 4 and sys.argv[1] == "capture":
        mode = "capture"
        window = parse_window_id(sys.argv[2])
        pid = parse_pid(sys.argv[3])
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
        windows = matching_windows(lib, display, pid)
        if mode == "close":
            send_close(lib, display, window, windows)
        elif mode == "capture":
            result = capture_window(lib, display, window, windows)
        lib.XSync(display, 0)
        if errors:
            raise HelperError("X11 protocol error")
        if mode == "inspect":
            result = {"windows": windows}
        elif mode == "close":
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
