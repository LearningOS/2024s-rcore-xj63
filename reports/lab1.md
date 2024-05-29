# 编程作业

引入一个新的系统调用 `sys_task_info` 以获取当前任务的信息

```
系统调用介绍：
syscall id 410
args {
  0: *TaskInfo
  1: Undefine
  2: Undefine
}
return 0:Ok 1:Err

将调用的结果保存到arg0中
TaskInfo {
  time: 从第一次任务开始到当前的时间（单位：ms）
  status: 任务运行的状态，默认为 Running
  syscall_times: 此任务系统调用的种类及次数，为一个数组，索引为系统调用号，值为调用的次数
}
```

# 简答作业

1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。
   请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b*bad*\*.rs) ），
   描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

```
[rustsbi] RustSBI version 0.4.0-alpha.1, adapting to RISC-V SBI v2.0.0
[kernel] Loading app_0
[kernel] PageFault in application, kernel killed it.              # 页错误，访问的地址可能不再可访问的区间
[kernel] Loading app_1
[kernel] IllegalInstruction in application, kernel killed it.     # 执行了非法操作, 使用 S 态特权指令
[kernel] Loading app_2
[kernel] IllegalInstruction in application, kernel killed it.     # 执行了非法操作，使用 S 态寄存器
```

2.  深入理解 trap.S 中两个函数 **alltraps 和 **restore 的作用，并回答如下问题:

    1.  L40：刚进入 **restore 时，a0 代表了什么值。请指出 **restore 的两种使用情景。
        刚进入 restore 时， a0 代表了程序栈,
        restore 用于将 \_\_alltraps 保存的寄存器恢复，用于系统调用返回，或者用于程序暂停后切换到该程序

    2.  L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

        ```asm
        ld t0, 32*8(sp)
        ld t1, 33*8(sp)
        ld t2, 2*8(sp)
        csrw sstatus, t0
        csrw sepc, t1
        csrw sscratch, t2
        ```

        sstatus: 状态寄存器，陷入前处于什么状态
        sepc: 陷入前的pc地址，用于返回该程序
        sscratch:用于保存内核或程序的栈指针，用于恢复用户栈

    3.  L50-L56：为何跳过了 x2 和 x4？

        ```asm
        ld x1, 1*8(sp)
        ld x3, 3*8(sp)
        .set n, 5
        .rept 27
           LOAD_GP %n
              .set n, n+1
        .endr
        ```

        x4: 与线程相关，目前还是单线程的，没有用到
        x2：栈指针，之前已经交换过了

    4.  L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？
        `csrrw sp, sscratch, sp`
        sp 为用户态程序的栈, sscratch 为内核态的栈

    5.  \_\_restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？
        `sret`: 当前处于 S 态， 执行 sret 恢复至 U 态，并根据 sepc 跳转到用户程序

    6.  L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？
        `csrrw sp, sscratch, sp`
        sscratch: 用户的栈
        sp: 内核的栈

    7.  从 U 态进入 S 态是哪一条指令发生的？
        `ecall`

# 荣誉准则

1.  在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

        [ai bing](https://www.bing.com/chat?q=Microsoft+Copilot&FORM=hpcodx)
        [codeium](https://codeium.com/)  // neovim 的补全

2.  此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

        [rCore-Tutorial-Book-v3](https://rcore-os.cn/rCore-Tutorial-Book-v3/chapter3/index.html)
        [rCore-Tutorial-Guide-2024S](https://learningos.cn/rCore-Tutorial-Guide-2024S/chapter3/index.html)

3.  我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4.  我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
