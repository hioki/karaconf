// Prints the bundle identifier of the frontmost app, now and whenever it
// changes, one per line (an empty line for an app without one).
// sync_frontmost_app.sh runs it over ssh on the Mac controlled through
// Screen Sharing.
import AppKit

func emit(_ app: NSRunningApplication?) {
    print(app?.bundleIdentifier ?? "")
    fflush(stdout)
}

emit(NSWorkspace.shared.frontmostApplication)
NSWorkspace.shared.notificationCenter.addObserver(
    forName: NSWorkspace.didActivateApplicationNotification, object: nil, queue: .main
) { notification in
    emit(notification.userInfo?[NSWorkspace.applicationUserInfoKey] as? NSRunningApplication)
}
RunLoop.main.run()
