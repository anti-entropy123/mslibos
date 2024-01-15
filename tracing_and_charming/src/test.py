import json
import matplotlib.pyplot as plt


def plot_boxplot(data1, data2, labels=None):
    # 创建箱形图
    plt.boxplot([data1, data2], labels=labels)

    # 添加标题和标签
    plt.title('Boxplot for Two Arrays')
    plt.xlabel('Array')
    plt.ylabel('Values')

    # 显示图形
    plt.show()


# 示例数据
with open("./media_service_trace.log") as f:
    array1 = json.loads(f.read())

with open("./media_service_no_libos_trace.log") as f:
    array2 = json.loads(f.read())

# 绘制箱形图
plot_boxplot(array1, array2, labels=['Array 1', 'Array 2'])
