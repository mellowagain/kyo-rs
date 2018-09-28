/*
 * kyo-rs - Rust rewrite of kyo, a modern osu! server switcher
 * Copyright (C) 2018 Marc3842h, czapek
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

"use strict";

function connectShiro() {
    invokeBackend(
        {
            cmd: "connect",
            address: document.getElementById("connect-address").value
        }
    );
}

function connectBancho() {
    invokeBackend(
        {
            cmd: "disconnect"
        }
    )
}

function installCertificate() {
    invokeBackend(
        {
            cmd: "install"
        }
    )
}

function invokeBackend(args) {
    window.external.invoke(JSON.stringify(args));
}
