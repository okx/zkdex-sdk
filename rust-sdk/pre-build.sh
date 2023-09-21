#帮我在centos中 下载安装 1.1.1g 版本的openssl
# 1. 下载
# 2. 解压
# 3. 配置
# 4. 编译
# 5. 安装
# 6. 验证
# 7. 设置环境变量
# 8. 删除源文件
# 9. 重启服务器
# 10. 验证是否安装成功
# 11. 卸载
# 12. 重启服务器
# 13. 验证是否卸载成功
# 14. 查看版本信息

# 1. 下载
wget https://www.openssl.org/source/openssl-1.1.1g.tar.gz
# 2. 解压
tar -zxvf openssl-1.1.1g.tar.gz
# 3. 配置
cd openssl-1.1.1g
./config --prefix=/usr/local/openssl
# 4. 编译
make
# 5. 安装
make install
# 6. 验证
/usr/local/openssl/bin/openssl version
# 7. 设置环境变量
echo "/usr/local/openssl/lib" >> /etc/ld.so.conf
ldconfig -v


