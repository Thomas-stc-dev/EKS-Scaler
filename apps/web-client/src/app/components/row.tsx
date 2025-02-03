'use client'
import { Card, Space, TimePicker, Button, Flex, Tag, Empty, message } from 'antd';
import dayjs, { Dayjs } from 'dayjs';
import { useMemo, useState } from 'react';
import { CloudServerOutlined, DownCircleOutlined, SaveOutlined, UpCircleOutlined } from '@ant-design/icons';
import { FilteredData, ScheduleType } from './container';
import { useRouter } from 'next/navigation'

const format = 'HH:mm';

export interface RowProps {
    schedules: FilteredData[keyof FilteredData],

}

export default function Row({ schedules }: RowProps) {    
    const router = useRouter();
    const [messageApi, contextHolder] = message.useMessage();

    const defaultSchedule = schedules.find(schedule => schedule.kind === 'default');
    const customSchedule = schedules.find(schedule => schedule.kind === 'custom');


    const disabled = defaultSchedule?.disabled;
    const defaultClusterName = defaultSchedule?.id.split('_')[0];
    // default times
    const defaultStartHour = defaultSchedule ? parseInt(defaultSchedule.start.split(':')[0]) : 9;
    const defaultEndHour = defaultSchedule ? parseInt(defaultSchedule.end.split(':')[0]) : 21;
    const defaultStartMinute = defaultSchedule ? parseInt(defaultSchedule.start.split(':')[1]) : 0;
    const defaultEndMinute = defaultSchedule ? parseInt(defaultSchedule.end.split(':')[1]) : 0;

    // custom times
    const customStartHour = customSchedule ? parseInt(customSchedule?.start.split(':')[0]) : 9;
    const customEndHour = customSchedule ? parseInt(customSchedule?.end.split(':')[0]) : 21;
    const customStartMinute = customSchedule ? parseInt(customSchedule?.start.split(':')[1]) : 0;
    const customEndMinute = customSchedule ? parseInt(customSchedule?.end.split(':')[1]) : 0;

    // using set before parse did not work
    // default
    const defaultStartTime = dayjs().set('hour', defaultStartHour).set('minute', defaultStartMinute);
    const defaultEndTime = dayjs().set('hour', defaultEndHour).set('minute', defaultEndMinute);
    // custom
    const customStartTime = dayjs().set('hour', customStartHour).set('minute', customStartMinute);
    const customEndTime = dayjs().set('hour', customEndHour).set('minute', customEndMinute);

    // default
    const [defaultStartValue, setDefaultStartValue] = useState<Dayjs | null>(defaultStartTime);
    const [defaultEndValue, setDefaultEndValue] = useState<Dayjs | null>(defaultEndTime);

    // custom
    const [customStartValue, setCustomStartValue] = useState<Dayjs | null>(customStartTime);
    const [customEndValue, setCustomEndValue] = useState<Dayjs | null>(customEndTime);

    const [isLoading, setIslLoading] = useState(false);
    const success = (message: string) => {
        messageApi.open({
            type: 'success',
            content: message,
        });
    };

    const error = (message: string) => {
        messageApi.open({
            type: 'error',
            content: message,
        });
    };



    const disableCustomSaveSchedule = useMemo(() => {
        if (customStartValue && customEndValue) {
            return customStartValue.hour() === customStartHour
                && customEndValue.hour() === customEndHour
                && customStartValue.minute() === customStartMinute
                && customEndValue.minute() === customEndMinute;
        }
        return !customStartValue || !customEndValue;
    }, [customStartValue, customEndValue, customStartHour, customEndHour, customStartMinute, customEndMinute]);

    const disableDefaultSaveSchedule = useMemo(() => {
        if (defaultStartValue && defaultEndValue) {
            return defaultStartValue.hour() === defaultStartHour
                && defaultEndValue.hour() === defaultEndHour
                && defaultStartValue.minute() === defaultStartMinute
                && defaultEndValue.minute() === defaultEndMinute;
        }
        return !defaultStartValue || !defaultEndValue;
    }, [defaultStartValue, defaultEndValue, defaultStartHour, defaultEndHour, defaultStartMinute, defaultEndMinute]);

    const saveSchedule = async (schedule: ScheduleType) => {
        // POST request to api endpoint
        const { kind, id } = schedule;
        try {
            setIslLoading(true);
            const response = await fetch('api/save-schedule', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    schedule: {
                        start: kind === 'default' ? defaultStartValue?.format('HH:mm') : customStartValue?.format('HH:mm'),
                        end: kind === 'default' ? defaultEndValue?.format('HH:mm') : customEndValue?.format('HH:mm'),
                        clusterName: id?.split('_')[0],
                        kind,
                        id
                    }
                }),

            });
            setIslLoading(false);
            console.log(response);
            success(`${kind} schedule saved successfully for ${id?.split('_')[0]}`);

            if (response.status === 200) {
                // Force refresh the page
                router.refresh();
            }
        } catch (err) {
            setIslLoading(false);
            error(err as string);
            console.log(error);
        }
    }

    const callScaler = async (type: string) => {
        try {
            setIslLoading(true);
            const response = await fetch('api/karpenter-lambda', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    type,
                    clusterName: defaultClusterName,
                }),

            });
            setIslLoading(false);

            const responseData = await response.json();
            if (responseData?.Payload?.statusCode === 200) {
                success(responseData?.Payload?.body.text);
                if (type !== 'up') {
                    // if previous reuest is 200 and is a scale down request
                    setIslLoading(true);
                    const response = await fetch('api/ec2-lambda', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                        },
                        body: JSON.stringify({
                            clusterName: defaultClusterName,
                        }),

                    });
                    setIslLoading(false);
                    if (response.status === 200) {
                        success('EC2 instances terminated successfully');
                    } else {
                        error('Unable to terminate EC2 instances');
                    }
                }
            } else {
                setIslLoading(false);
                error(`Unable to set Karpenter CPU Limit. Retry in a few minutes.`);

            }
        } catch (err) {
            setIslLoading(false);
            error(err as string);
            console.log(err);
        }
    }

    if (!defaultSchedule || !customSchedule) {
        const defaultClusterName = defaultSchedule ? defaultSchedule?.id.split('_')[0] : customSchedule?.id.split('_')[0];
        return (
            <Card title={defaultClusterName} hoverable style={{ marginTop: 24 }}>
                <Empty description={`${!customSchedule ? `No custom Schedule, check DB.` : `No default Schedule, check DB.`} `} />

            </Card>
        )
    }

    return (
        <div title={`${disabled ? `Disabled - edit DB to enable` : ``}`}>
            {contextHolder}


            <Card  title={`${defaultClusterName}${disabled ? ` - Disabled` : ``}`} hoverable style={{ marginTop: 24, opacity: disabled ? 0.2 : 1, background: disabled ? '#cccccc' : '' }}
            // actions={[
            //     <CloseCircleTwoTone twoToneColor="#D0342C" title='Cluster Off' />,
            //     <InfoCircleTwoTone twoToneColor="#E9D502" title='Cluster Status Unknown' />,
            //     <CheckCircleTwoTone twoToneColor="#52c41a" title='Cluster On' />
            // ]}
            >
                <Flex align='center' justify='space-between' className="my-4" >
                    <Flex vertical>

                        <Space.Compact block direction='vertical' >
                            <Space.Compact direction='horizontal' className="mb-4">
                                <Tag icon={<CloudServerOutlined />} color="#55acee">
                                    Custom
                                </Tag>
                            </Space.Compact>
                            <Space.Compact block direction='horizontal' >
                                <TimePicker.RangePicker defaultValue={[customStartTime, customEndTime]} format={format} minuteStep={15} size='large'
                                    onChange={(dates) => {
                                        setCustomStartValue(dates ? dates[0] : customStartTime);
                                        setCustomEndValue(dates ? dates[1] : customEndTime);
                                    }} />
                                <Button type="default" size='large' color="cyan" disabled={disableCustomSaveSchedule} icon={<SaveOutlined />} onClick={() => saveSchedule(customSchedule)} loading={isLoading}>Save Schedule</Button>
                            </Space.Compact>
                        </Space.Compact>

                        <Space.Compact block direction='vertical' >
                            <Space.Compact direction='horizontal' className="my-4">

                                <Tag icon={<CloudServerOutlined />} color="#52c41a">
                                    Default
                                </Tag>
                            </Space.Compact>

                            <Space.Compact block direction='horizontal' >
                                <TimePicker.RangePicker defaultValue={[defaultStartTime, defaultEndTime]} format={format} minuteStep={15} size='large'
                                    onChange={(dates) => {
                                        setDefaultStartValue(dates ? dates[0] : defaultStartTime);
                                        setDefaultEndValue(dates ? dates[1] : defaultEndTime);
                                    }} />
                                <Button type="default" size='large' disabled={disableDefaultSaveSchedule} icon={<SaveOutlined />} onClick={() => saveSchedule(defaultSchedule)} loading={isLoading}>Save Schedule</Button>
                            </Space.Compact>

                        </Space.Compact>

                    </Flex>
                    <Space.Compact direction='horizontal' >
                        <Button type="primary" size='large' color="cyan" title="Scale Up" icon={<UpCircleOutlined />} onClick={() => callScaler("up")} loading={isLoading}>Turn On</Button>
                        <Button type="default" size='large' color="cyan" title="Scale Down" icon={<DownCircleOutlined />} onClick={() => callScaler("down")} loading={isLoading}>Turn Off</Button>
                    </Space.Compact>
                </Flex>

            </Card>
        </div>
    )
}