pub fn assert_command_success(result: assert_cmd::assert::Assert) -> assert_cmd::assert::Assert {
    result.success()
}
