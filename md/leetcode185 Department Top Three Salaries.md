------------------
title = "leetcode185 Department Top Three Salaries"
publishTime = "2017-09-03 16:41:00"
id = "7471959"
tags = [ "算法", "leetcode",]

--------------

Employee表存储员工姓名、员工所在公寓、员工工资
Department表存储公寓id
评选出各个公寓的工资前三名的员工。

遇到的问题如下：
* limit，in等语句不能用在嵌套select语句中，多封装一层就可以了
* select子句如何访问外部的关系表，虽然可以直接访问外面第一层的，但是无法访问外面第二层的

```sql
select dep.name as Department,
       who.name as Employee,
       who.salary as Salary
from
     department as dep ,employee as who
     where dep.id=who.departmentid 
     and 2>=( select count(1) from (
        select distinct salary,departmentid from employee 
        )as dep_salary 
        where salary>who.salary and dep_salary.departmentid=dep.id
      )
order by Salary desc;
```
        