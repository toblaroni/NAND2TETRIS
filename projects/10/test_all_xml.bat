@echo off
echo "Testing ArrayTest..."
call ..\..\..\tools\TextComparer.bat ..\ArrayTest\Main.xml ..\ArrayTest\original_xml\Main.xml || goto :error

echo.
echo "Testing ExpressionLessSquare\Main.xml..."
call ..\..\..\tools\TextComparer.bat ..\ExpressionLessSquare\Main.xml ..\ExpressionLessSquare\original_xml\Main.xml || goto :error
echo "Testing ExpressionLessSquare\Square.xml..."
call ..\..\..\tools\TextComparer.bat ..\ExpressionLessSquare\Square.xml ..\ExpressionLessSquare\original_xml\Square.xml || goto :error
echo "Testing ExpressionLessSquare\SquareGame.xml..."
call ..\..\..\tools\TextComparer.bat ..\ExpressionLessSquare\SquareGame.xml ..\ExpressionLessSquare\original_xml\SquareGame.xml || goto :error

echo.
echo "Testing Square\Main.xml..."
call ..\..\..\tools\TextComparer.bat ..\Square\Main.xml ..\Square\original_xml\Main.xml || goto :error
echo "Testing Square\Square.xml..."
call ..\..\..\tools\TextComparer.bat ..\Square\Square.xml ..\Square\original_xml\Square.xml || goto :error
echo "Testing Square\SquareGame.xml..."
call ..\..\..\tools\TextComparer.bat ..\Square\SquareGame.xml ..\Square\original_xml\SquareGame.xml || goto :error

goto :end

:error
echo An error occurred while processing the batch script.
exit /b 1

:end
echo ---------------------------------------
echo Successfully compared all xml files! :)
