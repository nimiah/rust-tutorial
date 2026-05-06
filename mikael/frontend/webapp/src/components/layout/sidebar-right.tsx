"use client";

export function RightSidebar() {
  return (
    <div className="p-4 space-y-4">
      <h2 className="font-semibold text-lg">Contacts</h2>

      <div className="space-y-2">
        <div className="flex items-center gap-3">
          <div className="h-3 w-3 rounded-full bg-green-500" />

          <span>
            John Doe
          </span>
        </div>

        <div className="flex items-center gap-3">
          <div className="h-3 w-3 rounded-full bg-green-500" />

          <span>
            Jane Smith
          </span>
        </div>
      </div>
    </div>
  )
}
