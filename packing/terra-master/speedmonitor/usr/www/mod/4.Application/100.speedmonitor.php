<!doctype html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="description" content="">
    <title>Speed Monitor</title>
    <!-- Bootstrap core CSS -->
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/css/bootstrap.min.css" rel="stylesheet"
          integrity="sha384-EVSTQN3/azprG1Anm3QDgpJLIm9Nao0Yz1ztcQTwFspd3yD65VohhpuuCOmLASjC" crossorigin="anonymous">
    <script src="https://cdn.jsdelivr.net/npm/echarts@5.1.2/dist/echarts.min.js"
            integrity="sha256-TI0rIaxop+pDlHNVI6kDCFvmpxNYUnVH/SMjknZ/W0Y=" crossorigin="anonymous"></script>
    <script src="https://cdn.jsdelivr.net/npm/jquery@3.6.0/dist/jquery.min.js"
            integrity="sha256-/xUj+3OJU5yExlq6GSYGSHk7tPXikynS7ogEvDej/m4=" crossorigin="anonymous"></script>
    <meta name="theme-color" content="#7952b3">
    <style>

    </style>
</head>
<body>


<main>
    <section class="py-1 text-center container">
        <div class="row py-lg-1">
            <div id="main" style="width:100%;height:580px;"></div>

            <script type="text/javascript">
                // 基于准备好的dom，初始化echarts实例
                var myChart = echarts.init(document.getElementById('main'));

                // 指定图表的配置项和数据
                option = {
                    tooltip: {
                        trigger: 'axis'
                    },
                    legend: {
                        data: ['下载速度', '上传速度']
                    },
                    toolbox: {
                        show: true,
                        feature: {
                            magicType: {type: ['line', 'bar']},
                            saveAsImage: {}
                        }
                    },
                    xAxis: {
                        type: 'category',
                        boundaryGap: false,
                        data: []
                    },
                    yAxis: {
                        type: 'value',
                        axisLabel: {
                            formatter: '{value} Mbps'
                        }
                    },
                    series: [
                        {
                            name: '下载速度',
                            type: 'line',
                            data: [],
                            markPoint: {
                                data: [
                                    {type: 'max', name: '最大值'},
                                    {type: 'min', name: '最小值'}
                                ]
                            },
                            markLine: {
                                data: [
                                    {type: 'average', name: '平均值'}
                                ]
                            }
                        },
                        {
                            name: '上传速度',
                            type: 'line',
                            data: [],
                            markPoint: {
                                data: [
                                    {type: 'max', name: '最大值'},
                                    {type: 'min', name: '最小值'}
                                ]
                            },
                            markLine: {
                                data: [
                                    {type: 'average', name: '平均值'}
                                ]
                            }
                        }
                    ]
                };

                $.ajax({
                    <?php
                    $ip = $_SERVER['SERVER_ADDR'];
                    echo "url: 'http://$ip:32001/data',";
                    ?>
                    success: function (result) {
                        for (const r in result) {
                            option.xAxis.data.push('采集点:' + r);

                            option.series[0].data.push(result[r].download / 1000);
                            option.series[1].data.push(result[r].upload / 1000);
                        }
                        myChart.setOption(option);
                    }
                });
            </script>
        </div>
        <div class="row py-lg-1">
            <div class="row py-lg-1 col-4 offset-4">
                <button type="button" id="start-test" class="btn btn-primary">开始一次测试</button>

                <script type="text/javascript">
                    $("#start-test").click(function () {
                        $.ajax({
                            <?php
                            $ip = $_SERVER['SERVER_ADDR'];
                            echo "url: 'http://$ip:32001/test',";
                            ?>
                            success: function (result) {
                            }
                        });
                        $("#start-test").prop("disable", true)
                            .text("正在测试...")
                            .addClass("btn-secondary").removeClass("btn-primary");
                    });
                </script>
            </div>
        </div>
    </section>

</main>


<script src="https://cdn.jsdelivr.net/npm/bootstrap@5.0.2/dist/js/bootstrap.bundle.min.js"
        integrity="sha384-MrcW6ZMFYlzcLA8Nl+NtUVF0sA7MsXsP1UyJoMp4YLEuNSfAP+JcXn/tWtIaxVXM"
        crossorigin="anonymous"></script>

</body>
</html>
