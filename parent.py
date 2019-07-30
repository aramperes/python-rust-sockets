import subprocess
import os
import socket

s_parent, s_child = socket.socketpair()
fd_parent = s_parent.fileno()
fd_child = s_child.fileno()
proc = subprocess.Popen(["./target/debug/socket_child", str(fd_child)], pass_fds=(fd_child,))

print("[Python] Received:", s_parent.recv(256))
s_parent.send(b"Welcome child")

s_parent.close()
