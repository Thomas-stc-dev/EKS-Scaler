
import React from 'react';
import { Resource } from "sst";
import { DynamoDBClient, QueryCommandOutput, ScanCommand } from "@aws-sdk/client-dynamodb";
import { Divider, Layout } from 'antd';
import Row from './row';

const client = new DynamoDBClient();
export interface ScheduleType {
  id: string;
  kind: string;
  start: string;
  end: string;
  disabled: boolean;

}
export interface FilteredData {
  [key: string]: ScheduleType[]
}

export default async function Container() {

  const getData = async () => {
    const params = {
      TableName: Resource['EKS-Scaler-Table'].name,
    };
    const data: QueryCommandOutput = await client.send(new ScanCommand(params));
    return data;
  }

  const filterData = (data: QueryCommandOutput): FilteredData => {
    const filteredData: FilteredData = {};
    data.Items?.forEach((item) => {      
      const cluster = item.cluster['S'] as string;
      const start = item.start['S'] || '09:00';
      const end = item.end['S'] || '21:00';
      const kind = item.kind['S'] || 'default';
      const id = item.id['S'] || '0';
      const disabled = item.disabled['BOOL'] || false;
      // if custom then replace existing default
      if (!filteredData[cluster]) {
        filteredData[cluster] = [{
          id,
          start,
          end,
          kind,
          disabled
        }];
      } else {
        filteredData[cluster].push({
          id,
          start,
          end,
          kind,
          disabled
        }
        )
      }
    });

    return filteredData;
  };

  const filteredData = filterData(await getData());
  const clusters = Object.keys(filteredData);
  const schedules = Object.values(filteredData);  

  return (
    <Layout style={{ padding: '24px 50px', background: '#fff' }} className='max-w-5xl m-auto' >
      <Divider style={{ borderColor: '#1677ff' }}>EKS Scaler</Divider>
      {clusters.map((clusterName, k) => {
        return (
          <Row key={k} schedules={schedules[k]} />
        )
      })}

    </Layout>
  )
}

