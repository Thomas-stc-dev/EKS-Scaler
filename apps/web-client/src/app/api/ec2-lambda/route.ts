import { LambdaClient, ListLayersCommand, InvokeCommand, GetFunctionUrlConfigCommand } from "@aws-sdk/client-lambda";
import Joi from 'joi';
import { Resource } from 'sst';

const schema = Joi.object({
    clusterName: Joi.string().required(),
});

const client = new LambdaClient();


export async function POST(request: Request) {
    // get schedule data from request and parse to json    
    const reqParams = await request.json();
    const { clusterName } = reqParams;

    const lambdaFunctionName = Resource['TerminateEc2'].functionName;
    if (request.method === 'POST') {
        const { error } = schema.validate(reqParams);
   
        
        if (error) {
            return new Response(JSON.stringify({ error: error.details[0].message }), { status: 400 });
        }

        try {
            const getUrlCommand = new GetFunctionUrlConfigCommand({
                FunctionName: lambdaFunctionName,
            });
            const functionUrlResponse = await client.send(getUrlCommand);
            const functionUrl = functionUrlResponse.FunctionUrl;
            const requestOptions = {
                method: "GET",
                redirect: "follow" as RequestRedirect
              };
            const url = `${functionUrl}/?cluster-name=${clusterName}`;
            const response = await fetch(url, requestOptions)            
            return response;
        } catch (error) {
            return new Response(JSON.stringify({ error }), { status: 400 });

        }


    }


    return new Response(JSON.stringify({ error: 'Method not allowed' }), { status: 405 });

} 
