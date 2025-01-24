import { NextApiRequest } from 'next';
import { DynamoDBClient, QueryCommandOutput, ScanCommand, ScanCommandOutput, PutItemCommand } from "@aws-sdk/client-dynamodb";
import Joi from 'joi';
import { Resource } from 'sst';

const scheduleSchema = Joi.object({

    start: Joi.string().required(),
    end: Joi.string().required(),
    clusterName: Joi.string().required(),
    kind: Joi.string().required(),
    id: Joi.string().required(),

});

const client = new DynamoDBClient();


export async function POST(request: Request) {
    // get schedule data from request and parse to json    
    const { schedule } = await request.json()
    const { start, end, kind, id, clusterName } = schedule;

    if (request.method === 'POST') {
        const { error } = scheduleSchema.validate(schedule);

        if (error) {
            return new Response(JSON.stringify({ error: error.details[0].message }), { status: 400 });
        }

        const params = {
            TableName: Resource['EKS-Scaler-Table'].name,
            Item: {
                cluster: { S: clusterName },
                id: { S: id },
                start: { S: start },
                end: { S: end },
                kind: { S: kind }
            }
        };
        try {
            const data = await client.send(new PutItemCommand(params));
            return new Response(JSON.stringify({ data }), { status: 200 });

        } catch (error) {
            return new Response(JSON.stringify({ error }), { status: 400 });

        }


    }


    return new Response(JSON.stringify({ error: 'Method not allowed' }), { status: 405 });

} 
