# 用户管理API

## 添加用户
- 方法: GET
- 路径: `/add_user`
- 参数:
  - `name`: 姓名
  - `student_id`: 学号
- 示例: `/add_user?name=张三&student_id=20230001`

## 删除用户
- 方法: GET
- 路径: `/delete_user`
- 参数:
  - `id`: 用户ID
- 示例: `/delete_user?id=1`

## 更新用户
- 方法: GET
- 路径: `/update_user`
- 参数:
  - `id`: 用户ID
  - `name`: 新姓名
  - `student_id`: 新学号
- 示例: `/update_user?id=1&name=李四&student_id=20230002`

## 查询用户

### 查询单个用户
- 方法: GET
- 路径: `/query_user`
- 参数:
  - `id`: 用户ID
- 示例: `/query_user?id=1`

### 查询所有用户
- 方法: GET
- 路径: `/query_user`
- 参数: 无
- 示例: `/query_user`