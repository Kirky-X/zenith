// Copyright (c) 2025 Kirky.X
//
// Licensed under the MIT License
// See LICENSE file in the project root for full license information.

pub fn assert_command_success(result: assert_cmd::assert::Assert) -> assert_cmd::assert::Assert {
    result.success()
}
