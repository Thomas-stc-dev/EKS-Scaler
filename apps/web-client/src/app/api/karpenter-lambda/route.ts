import { LambdaClient, ListLayersCommand, InvokeCommand } from "@aws-sdk/client-lambda";
import Joi from 'joi';
import { Resource } from 'sst';

const schema = Joi.object({
    clusterName: Joi.string().required(),
    type: Joi.string().required(),
});

const client = new LambdaClient();


export async function POST(request: Request) {
    // get schedule data from request and parse to json    
    const reqParams = await request.json();
    const { type, clusterName } = reqParams;

    const lambdaFunctionName = Resource['KarpenterCPU'].functionName;
    if (request.method === 'POST') {
        const { error } = schema.validate(reqParams);


        if (error) {
            return new Response(JSON.stringify({ error: error.details[0].message }), { status: 400 });
        }

        const params = {
            'queryStringParameters': {
                'cluster-name': `${clusterName}`,
                'cpu-limit': type === 'up' ? '1000' : '0'
            }
        };
        const command = new InvokeCommand({
            FunctionName: lambdaFunctionName,
            Payload: Buffer.from(JSON.stringify(params), 'utf8')
        });
        const response = await client.send(command);

        try {
            // const data = await client.send(new PutItemCommand(params));
            if (response?.FunctionError) {
                return new Response(JSON.stringify({ error: response?.FunctionError }), { status: 400 });
            }
            if (response?.Payload) {
                const jsonString = Buffer.from(response?.Payload).toString('utf8')
                const parsedData = JSON.parse(jsonString)
                return new Response(JSON.stringify({ ...response, Payload: { ...parsedData, body: JSON.parse(parsedData.body) } }), { status: parsedData.statusCode });

            }
            return new Response(JSON.stringify({ response }), { status: 200 });

        } catch (error) {
            return new Response(JSON.stringify({ error }), { status: 400 });

        }


    }


    return new Response(JSON.stringify({ error: 'Method not allowed' }), { status: 405 });

} 
