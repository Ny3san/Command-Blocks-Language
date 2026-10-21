@echo off
setlocal EnableExtensions EnableDelayedExpansion
set GIT_PAGER=cat
set PAGER=cat

:: Publica codigo e documentacao do Command Blocks no GitHub.
:: Execute este arquivo dentro da pasta D:\pinto ou coloque-o na raiz do projeto.

title Publicar Command Blocks no GitHub
color 0B

cd /d "%~dp0"

echo.
echo ===============================================
echo   PUBLICAR COMMAND BLOCKS NO GITHUB
echo ===============================================
echo.

git rev-parse --show-toplevel >nul 2>&1
if errorlevel 1 (
    echo ERRO: esta pasta nao e um repositorio Git.
    echo Coloque este .bat dentro de D:\pinto e tente novamente.
    pause
    exit /b 1
)

for /f "delims=" %%R in ('git rev-parse --show-toplevel') do set "ROOT=%%R"
cd /d "%ROOT%"

echo Repositorio: %ROOT%
echo.

echo [1/6] Atualizando .gitignore...
>> .gitignore echo.
>> .gitignore echo # Arquivos locais e gerados - Command Blocks
>> .gitignore echo target/
>> .gitignore echo *.rar
>> .gitignore echo *.vsix
>> .gitignore echo *.exe
>> .gitignore echo *.bak
>> .gitignore echo .vscode/
>> .gitignore echo node_modules/

if not exist "vscode-command-blocks" (
    echo AVISO: a pasta vscode-command-blocks nao existe.
    echo A extensao do VS Code nao sera publicada neste commit.
    echo.
)

echo [2/6] Adicionando codigo e documentacao...
git add .
if errorlevel 1 (
    echo ERRO ao adicionar os arquivos.
    pause
    exit /b 1
)

:: Retira explicitamente arquivos proibidos do staging, mesmo se ja estiverem rastreados.
git restore --staged --quiet -- target 2>nul
git restore --staged --quiet -- "*.rar" 2>nul
git restore --staged --quiet -- "*.vsix" 2>nul
git restore --staged --quiet -- "*.exe" 2>nul
git restore --staged --quiet -- "*.bak" 2>nul

echo.
echo [3/6] Arquivos que serao publicados:
echo -----------------------------------------------
git --no-pager diff --cached --name-status
echo -----------------------------------------------
echo.

git diff --cached --quiet
if not errorlevel 1 (
    echo Nao ha alteracoes novas para publicar.
    git --no-pager status --short
    pause
    exit /b 0
)

choice /C SN /N /M "Continuar com commit? [S/N] "
if errorlevel 2 (
    echo Operacao cancelada. Nenhum commit foi criado.
    git reset >nul
    pause
    exit /b 0
)

echo.
echo [4/6] Criando commit...
git commit -m "feat: add VS Code support and documentation"
if errorlevel 1 (
    echo ERRO: o commit falhou.
    pause
    exit /b 1
)

echo.
echo [5/6] Verificando remote...
git remote get-url origin >nul 2>&1
if errorlevel 1 (
    echo ERRO: nenhum remote chamado origin foi configurado.
    echo Configure com:
    echo git remote add origin https://github.com/SEU_USUARIO/SEU_REPOSITORIO.git
    pause
    exit /b 1
)

for /f "delims=" %%B in ('git branch --show-current') do set "BRANCH=%%B"
echo Branch atual: !BRANCH!
echo.

git --no-pager status --short
choice /C SN /N /M "Enviar a branch !BRANCH! para o GitHub? [S/N] "
if errorlevel 2 (
    echo Commit criado, mas o push foi cancelado.
    pause
    exit /b 0
)

echo.
echo [6/6] Enviando para o GitHub...
git push -u origin "!BRANCH!"
if errorlevel 1 (
    echo ERRO no push. O commit local foi preservado.
    echo Verifique login, remote e permissoes do repositorio.
    pause
    exit /b 1
)

echo.
echo ===============================================
echo PUBLICADO COM SUCESSO NA BRANCH !BRANCH!
echo ===============================================
echo.
pause
exit /b 0
