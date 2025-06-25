!macro NSIS_HOOK_POSTINSTALL

  DetailPrint "Creating Python interpreter directory..."
  CreateDirectory "$INSTDIR\python\interpreter"

  DetailPrint "Extracting Python interpreter..."
  nsExec::ExecToLog 'powershell -NoProfile -ExecutionPolicy Bypass -Command "Expand-Archive -Path \"$INSTDIR\zip\interpreter.zip\" -DestinationPath \"$INSTDIR\python\interpreter\" -Force"'

  DetailPrint "Creating VSCode editor directory..."
  CreateDirectory "$INSTDIR\editor\vscode"

  DetailPrint "Extracting VSCode editor..."
  nsExec::ExecToLog 'powershell -NoProfile -ExecutionPolicy Bypass -Command "Expand-Archive -Path \"$INSTDIR\zip\vscode.zip\" -DestinationPath \"$INSTDIR\editor\vscode\" -Force"'

  DetailPrint "Deleting zip directory..."
  RMDir /r "$INSTDIR\zip"

!macroend