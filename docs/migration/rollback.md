# bkmrx SQLite 迁移回滚

回滚目标是恢复旧 App 的可用状态，同时保留新库和所有迁移证据。

## 保证

- 旧数据库 `/Users/gyf/.config/bkmr/bkmr.db` 在迁移期间只读，绝不原地更新；
- 时间戳备份目录保留 SQLite snapshot、完整业务 JSON、manifest 和迁移报告；
- 安装新版前会保留旧 `/Applications/bkmrx.app` 的时间戳副本；
- 回滚不删除新数据库。

## 新版验证失败时

1. 退出新版 App，并确认端口停止：

```bash
lsof -nP -iTCP:8733 -sTCP:LISTEN
```

2. 隔离新数据库，不要删除：

```bash
mv \
  '/Users/gyf/Library/Application Support/com.bkmrx/bookmarks.db' \
  '/Users/gyf/Library/Application Support/com.bkmrx/bookmarks.db.failed-<timestamp>'
```

同时移动同名 `-wal`、`-shm` 文件（如果存在），使用相同时间戳。

3. 将已备份的旧 App 恢复到 `/Applications/bkmrx.app`。实际备份路径会记录在安装日志中：

```bash
mv /Applications/bkmrx.app /Applications/bkmrx.app.failed-<timestamp>
cp -R '<旧 App 备份绝对路径>' /Applications/bkmrx.app
```

4. 启动旧 App，确认它仍读取：

```text
/Users/gyf/.config/bkmr/bkmr.db
```

5. 对旧库重新执行只读 hash、integrity 和数量检查，并与迁移报告比较。

## 修复后重新迁移

1. 保留失败的新库和原迁移目录；
2. 修复迁移工具并重新运行全部 fixture 测试；
3. 创建新的时间戳备份目录；
4. 确认目标 `bookmarks.db` 和 `.migrating` 都不存在；
5. 按 [runbook](runbook.md) 从头执行；
6. 新旧两次迁移报告分别保留。

不要用失败的新库覆盖旧 BKMR 数据库，也不要为了重试而删除旧 App、旧库或历史备份。
