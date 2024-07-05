<template>
  <el-auto-resizer>
    <template #default="{ height, width }">
      <el-table-v2 :columns="columns" :data="data" :width="width" :height="height" fixed>
        <template #empty>
          <div>No data</div>
        </template>
      </el-table-v2>
    </template>
  </el-auto-resizer>
</template>

<script lang="tsx" setup>
import { ref } from 'vue'

const columnNames = [
  '',
  'ID',
  'URL',
  'Method',
  'Status',
  'Code',
  'Time',
  'Duration',
  'Request',
  'Response'
]
const columns = ref(
  columnNames.map((name) => {
    let width = 128
    if (name == '') {
      width = 50
    } else if (name == 'URL') {
      width = 400
    } else if (name == 'Time') {
      width = 250
    }

    function cellRenderer(name: string) {
      if (name == '') {
        return function (data: any) {
          let code = data['rowData']['Code']
          let status = Math.floor(code / 100)
          if (status <= 2) {
            return (
              <div style="width: 100%; display: flex; justify-content: center; align-items: center;">
                <div style=" border-radius: 50%; width: 10px; height: 10px; background-color: var(--el-color-success)"></div>
              </div>
            )
          }
          if (status <= 4) {
            return (
              <div style="width: 100%; display: flex; justify-content: center; align-items: center;">
                <div style=" border-radius: 50%; width: 10px; height: 10px; background-color: var(--el-color-warning)"></div>
              </div>
            )
          }
          return (
            <div style="width: 100%; display: flex; justify-content: center; align-items: center;">
              <div style=" border-radius: 50%; width: 10px; height: 10px; background-color: var(--el-color-danger)"></div>
            </div>
          )
        }
      }
      if (name == 'Time') {
        return function (data: any) {
          function n(n: number) {
            if (!Number.isInteger(n) || n < 0) return n.toString()
            return n > 9 ? '' + n : '0' + n
          }

          function t(n: number) {
            if (n < 12) return 'AM'
            return 'PM'
          }

          let date = new Date(data['rowData']['Time'])
          return `${n(date.getHours())}:${n(date.getMinutes())}:${n(date.getSeconds())} ${t(date.getHours())}`
        }
      }
      if (name == 'Method') {
        return function (data: any) {
          return (
            <el-button type="primary" plain disabled style="cursor: unset;">
              {data['rowData']['Method']}
            </el-button>
          )
        }
      }
      return null
    }

    return {
      key: name,
      dataKey: name,
      title: name,
      width: width,
      cellRenderer: cellRenderer(name)
    }
  })
)

const data = ref([
  {
    id: '0',
    ID: '1',
    URL: 'http://localhost:8080/api/v1/requests/1',
    Method: 'POST',
    Status: 'Pending',
    Code: 201,
    Time: '2022-09-26T14:32:48Z',
    Duration: 60,
    Request: '16 KB',
    Response: '7KB'
  },
  {
    id: '1',
    ID: '2',
    URL: 'http://localhost:8080/api/v1/requests/2',
    Method: 'PUT',
    Status: 'Completed',
    Code: 201,
    Time: '2020-06-26T19:13:20Z',
    Duration: 62,
    Request: '8 KB',
    Response: '7KB'
  },
  {
    id: '2',
    ID: '3',
    URL: 'http://localhost:8080/api/v1/requests/3',
    Method: 'DELETE',
    Status: 'Failed',
    Code: 201,
    Time: '2022-04-26T07:26:29Z',
    Duration: 252,
    Request: '18 KB',
    Response: '2KB'
  },
  {
    id: '3',
    ID: '4',
    URL: 'http://localhost:8080/api/v1/requests/4',
    Method: 'DELETE',
    Status: 'Failed',
    Code: 500,
    Time: '2021-02-28T12:18:54Z',
    Duration: 224,
    Request: '9 KB',
    Response: '9KB'
  },
  {
    id: '4',
    ID: '5',
    URL: 'http://localhost:8080/api/v1/requests/5',
    Method: 'GET',
    Status: 'Completed',
    Code: 201,
    Time: '2022-09-19T03:19:42Z',
    Duration: 77,
    Request: '4 KB',
    Response: '1KB'
  },
  {
    id: '5',
    ID: '6',
    URL: 'http://localhost:8080/api/v1/requests/6',
    Method: 'DELETE',
    Status: 'Failed',
    Code: 201,
    Time: '2022-07-31T08:50:38Z',
    Duration: 458,
    Request: '13 KB',
    Response: '8KB'
  },
  {
    id: '6',
    ID: '7',
    URL: 'http://localhost:8080/api/v1/requests/7',
    Method: 'GET',
    Status: 'Failed',
    Code: 500,
    Time: '2021-05-20T03:16:14Z',
    Duration: 294,
    Request: '11 KB',
    Response: '5KB'
  },
  {
    id: '7',
    ID: '8',
    URL: 'http://localhost:8080/api/v1/requests/8',
    Method: 'POST',
    Status: 'Pending',
    Code: 404,
    Time: '2020-01-05T00:43:50Z',
    Duration: 465,
    Request: '5 KB',
    Response: '4KB'
  },
  {
    id: '8',
    ID: '9',
    URL: 'http://localhost:8080/api/v1/requests/9',
    Method: 'PUT',
    Status: 'Completed',
    Code: 404,
    Time: '2021-09-27T04:17:45Z',
    Duration: 168,
    Request: '10 KB',
    Response: '9KB'
  },
  {
    id: '9',
    ID: '10',
    URL: 'http://localhost:8080/api/v1/requests/10',
    Method: 'PUT',
    Status: 'Completed',
    Code: 201,
    Time: '2023-11-28T23:53:17Z',
    Duration: 198,
    Request: '12 KB',
    Response: '1KB'
  },
  {
    id: '10',
    ID: '11',
    URL: 'http://localhost:8080/api/v1/requests/11',
    Method: 'POST',
    Status: 'Completed',
    Code: 404,
    Time: '2021-05-28T05:01:47Z',
    Duration: 85,
    Request: '3 KB',
    Response: '4KB'
  },
  {
    id: '11',
    ID: '12',
    URL: 'http://localhost:8080/api/v1/requests/12',
    Method: 'POST',
    Status: 'Pending',
    Code: 500,
    Time: '2022-02-10T16:39:14Z',
    Duration: 57,
    Request: '3 KB',
    Response: '5KB'
  },
  {
    id: '12',
    ID: '13',
    URL: 'http://localhost:8080/api/v1/requests/13',
    Method: 'PUT',
    Status: 'Failed',
    Code: 500,
    Time: '2022-06-05T20:37:05Z',
    Duration: 77,
    Request: '1 KB',
    Response: '1KB'
  },
  {
    id: '13',
    ID: '14',
    URL: 'http://localhost:8080/api/v1/requests/14',
    Method: 'GET',
    Status: 'Completed',
    Code: 404,
    Time: '2020-09-27T04:17:45Z',
    Duration: 168,
    Request: '10 KB',
    Response: '9KB'
  },
  {
    id: '14',
    ID: '15',
    URL: 'http://localhost:8080/api/v1/requests/15',
    Method: 'POST',
    Status: 'Completed',
    Code: 200,
    Time: '2023-03-05T10:19:41Z',
    Duration: 112,
    Request: '10 KB',
    Response: '1KB'
  },
  {
    id: '15',
    ID: '16',
    URL: 'http://localhost:8080/api/v1/requests/16',
    Method: 'POST',
    Status: 'Completed',
    Code: 500,
    Time: '2021-08-01T23:29:41Z',
    Duration: 308,
    Request: '6 KB',
    Response: '7KB'
  },
  {
    id: '16',
    ID: '17',
    URL: 'http://localhost:8080/api/v1/requests/17',
    Method: 'PUT',
    Status: 'Completed',
    Code: 200,
    Time: '2022-01-06T20:53:23Z',
    Duration: 105,
    Request: '14 KB',
    Response: '1KB'
  },
  {
    id: '17',
    ID: '18',
    URL: 'http://localhost:8080/api/v1/requests/18',
    Method: 'PUT',
    Status: 'Pending',
    Code: 500,
    Time: '2020-05-23T14:29:06Z',
    Duration: 124,
    Request: '19 KB',
    Response: '9KB'
  },
  {
    id: '18',
    ID: '19',
    URL: 'http://localhost:8080/api/v1/requests/19',
    Method: 'PUT',
    Status: 'Completed',
    Code: 201,
    Time: '2020-08-09T14:16:35Z',
    Duration: 304,
    Request: '5 KB',
    Response: '8KB'
  },
  {
    id: '19',
    ID: '20',
    URL: 'http://localhost:8080/api/v1/requests/20',
    Method: 'DELETE',
    Status: 'Failed',
    Code: 404,
    Time: '2023-11-22T01:10:02Z',
    Duration: 60,
    Request: '7 KB',
    Response: '9KB'
  }
])
</script>
