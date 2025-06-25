# 下载所有依赖包,以便离线安装

```
./interpreter_3.13.3_macos/install/bin/pip download ./kiwi-0.1.0-py3-none-any.whl -d ./offline_packages
```

# 安装命令

```
pip install --no-index --find-links=./offline_packages my_package-1.0.0-py3-none-any.whl

```
