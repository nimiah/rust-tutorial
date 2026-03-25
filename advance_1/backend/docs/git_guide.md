### Yêu cầu:

1. Không được phép đưa các commit từ local branch lên master branch

### Flow:

1. Mỗi dev sẽ có một branch riêng, bằng cách tạo branch theo tên riêng từ local main branch
2. Khi có thay đổi cần commit:
   - Mô tả sơ về commit làm gì
   - Commit lên local branch

3. Nếu local branch chưa publish, thì publish

4. Tao Pull request đề merge từ branch của mình lên master
5. Sau khi MR được merge (code merge về master) thì nên pull các thay đổi về local branch (master branch)

6. Khi chúng ta muốn tiếp tục làm việc trên branch của mình, chúng ta phải rebase branch của mình lên master, sau khi master branch được update code mới nhất

`sh
git rebase main
git push -f
`
