lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("desk_tip", "Your desktop can be accessed with this ID and password."),
        ("connecting_status", "Connecting to the RustDesk network..."),
        ("not_ready_status", "Not ready. Please check your connection"),
        ("id_change_tip", "Only a-z, A-Z, 0-9 and _ (underscore) characters allowed. The first letter must be a-z, A-Z. Length between 6 and 16."),
        ("install_tip", "Due to UAC, RustDesk can not work properly as the remote side in some cases. To avoid UAC, please click the button below to install RustDesk to the system."),
        ("config_acc", "In order to control your Desktop remotely, you need to grant RustDesk \"Accessibility\" permissions."),
        ("config_screen", "In order to access your Desktop remotely, you need to grant RustDesk \"Screen Recording\" permissions."),
        ("agreement_tip", "By starting the installation, you accept the license agreement."),
        ("not_close_tcp_tip", "Don't close this window while you are using the tunnel"),
        ("setup_server_tip", "For faster connection, please set up your own server"),
        ("Auto Login", "Auto Login (Only valid if you set \"Lock after session end\")"),
        ("whitelist_tip", "Only whitelisted IP can access me"),
        ("whitelist_sep", "Separated by comma, semicolon, spaces or new line"),
        ("Wrong credentials", "Wrong username or password"),
        ("invalid_http", "must start with http:// or https://"),
        ("install_daemon_tip", "For starting on boot, you need to install system service."),
        ("android_input_permission_tip1", "In order for a remote device to control your Android device via mouse or touch, you need to allow RustDesk to use the \"Accessibility\" service."),
        ("android_input_permission_tip2", "Please go to the next system settings page, find and enter [Installed Services], turn on [RustDesk Input] service."),
        ("android_new_connection_tip", "New control request has been received, which wants to control your current device."),
        ("android_service_will_start_tip", "Turning on \"Screen Capture\" will automatically start the service, allowing other devices to request a connection to your device."),
        ("android_stop_service_tip", "Closing the service will automatically close all established connections."),
        ("android_version_audio_tip", "The current Android version does not support audio capture, please upgrade to Android 10 or higher."),
        ("android_start_service_tip", "Tap [Start Service] or enable [Screen Capture] permission to start the screen sharing service."),
        ("android_permission_may_not_change_tip", "Permissions for established connections may not be changed instantly until reconnected."),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("doc_fix_wayland", "https://rustdesk.com/docs/en/manual/linux/#x11-required"),
        ("server_not_support", "Not yet supported by the server"),
        ("android_open_battery_optimizations_tip", "If you want to disable this feature, please go to the next RustDesk application settings page, find and enter [Battery], Uncheck [Unrestricted]"),
        ("remote_restarting_tip", "Remote device is restarting, please close this message box and reconnect with permanent password after a while"),
        ("Are you sure to close the connection?", "Are you sure you want to close the connection?"),
        ("elevated_foreground_window_tip", "The current window of the remote desktop requires higher privilege to operate, so it's unable to use the mouse and keyboard temporarily. You can request the remote user to minimize the current window, or click elevation button on the connection management window. To avoid this problem, it is recommended to install the software on the remote device."),
        ("JumpLink", "View"),
        ("Stop service", "Stop Service"),
        ("hide_cm_tip", "Allow hiding only if accepting sessions via password and using permanent password"),
        ("wayland_experiment_tip", "Wayland support is in experimental stage, please use X11 if you require unattended access."),
        ("Slogan_tip", "Made with heart in this chaotic world!"),
        ("verification_tip", "A verification code has been sent to the registered email address, enter the verification code to continue logging in."),
        ("software_render_tip", "If you're using Nvidia graphics card under Linux and the remote window closes immediately after connecting, switching to the open-source Nouveau driver and choosing to use software rendering may help. A software restart is required."),
        ("config_input", "In order to control remote desktop with keyboard, you need to grant RustDesk \"Input Monitoring\" permissions."),
        ("request_elevation_tip", "You can also request elevation if there is someone on the remote side."),
        ("wait_accept_uac_tip", "Please wait for the remote user to accept the UAC dialog."),
        ("still_click_uac_tip", "Still requires the remote user to click OK on the UAC window of running RustDesk."),
        ("config_microphone", "In order to speak remotely, you need to grant RustDesk \"Record Audio\" permissions."),
        ("relay_hint_tip", "It may not be possible to connect directly, you can try to connect via relay. \nIn addition, if you want to use relay on your first try, you can add the \"/r\" suffix to the ID, or select the option \"Always connect via relay\" in the peer card."),
        ("No transfers in progress", ""),
        ("idd_driver_tip", "Install virtual display driver which is used when you have no physical displays."),
        ("confirm_idd_driver_tip", "The option to install the virtual display driver is checked. Note that a test certificate will be installed to trust the virtual display driver. This test certificate will only be used to trust Rustdesk drivers."),
        ("empty_recent_tip", "Oops, no recent sessions!\nTime to plan a new one."),
        ("empty_favorite_tip", "No favorite peers yet?\nLet's find someone to connect with and add it to your favorites!"),
        ("empty_lan_tip", "Oh no, it looks like we haven't discovered any peers yet."),
        ("empty_address_book_tip", "Oh dear, it appears that there are currently no peers listed in your address book."),
        ("identical_file_tip", "This file is identical with the peer's one."),
        ("show_monitors_tip", "Show monitors in toolbar"),
        ("enter_rustdesk_passwd_tip", "Enter RustDesk password"),
        ("remember_rustdesk_passwd_tip", "Remember RustDesk password"),
        ("login_linux_tip", "You need to login to remote Linux account to enable a X desktop session"),
        ("verify_rustdesk_password_tip", "Verify RustDesk password"),
        ("remember_account_tip", "Remember this account"),
        ("os_account_desk_tip", "This account is used to login the remote OS and enable the desktop session in headless"),
        ("another_user_login_title_tip", "Another user already logged in"),
        ("another_user_login_text_tip", "Disconnect"),
        ("xorg_not_found_title_tip", "Xorg not found"),
        ("xorg_not_found_text_tip", "Please install Xorg"),
        ("no_desktop_title_tip", "No desktop is available"),
        ("no_desktop_text_tip", "Please install GNOME desktop"),
        ("resolution_original_tip", "Original resolution"),
        ("resolution_fit_local_tip", "Fit local resolution"),
        ("resolution_custom_tip", "Custom resolution"),
        ("accept_and_elevate_btn_tooltip", "Accept the connection and elevate UAC permissions."),
        ].iter().cloned().collect();
}
