/*******************************************************************************
 * Copyright (c) 2024.
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the
 * Free Software Foundation, either version 3 of the License, or (at your
 * option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General
 * Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program. If not, see <https://www.gnu.org/licenses/>..
 ******************************************************************************/

/******************************************************************************
    Author:  
    Email: jb@taunais.com 
    Date: 17/5/24
 ******************************************************************************/

pub const WEBSOCKET_URL: &str = "ws://localhost:5678";
pub const MONGODB_URI: &str = "mongodb://localhost:27017";
pub const MONGODB_AUTH_SOURCE: &str = "admin";
pub const MONGODB_AUTH_MECHANISM: &str = "SCRAM-SHA-256";

pub const MECHANISM_SCRAM_SHA_256: &str = "SCRAM-SHA-256";

pub const MECHANISM_SCRAM_SHA_1: &str = "SCRAM-SHA-1";

pub const MECHANISM_MONGODB_CR: &str = "MONGODB-CR";

pub const MECHANISM_PLAIN: &str = "PLAIN";

pub const MECHANISM_MONGODB_AWS: &str = "MONGODB_AWS";

pub const MECHANISM_GSSAPI: &str = "GSSAPI";

pub const MECHANISM_MONGODB_X509: &str = "MONGODB-X509";
