import type {
	IExecuteFunctions,
	INodeType,
	INodeTypeDescription,
	INodeTypeBaseDescription,
	INodeExecutionData,
	ILoadOptionsFunctions,
	INodePropertyOptions,
} from 'n8n-workflow';

export class PromptKaizenV1 implements INodeType {
	description: INodeTypeDescription;

	constructor(baseDescription: INodeTypeBaseDescription) {
		this.description = {
			...baseDescription,
			version: 1,
			defaults: {
				name: 'Prompt Kaizen',
			},
			inputs: ['main'],
			outputs: ['main'],
			credentials: [
				{
					name: 'promptKaizenApi',
					required: true,
				},
			],
			properties: [
				{
					displayName: 'Operation',
					name: 'operation',
					type: 'options',
					noDataExpression: true,
					options: [
						{
							name: 'Get Prompt',
							value: 'getPrompt',
							description: 'Retrieve and render a prompt',
						},
						{
							name: 'Submit Feedback',
							value: 'submitFeedback',
							description: 'Submit feedback for a prompt version',
						},
					],
					default: 'getPrompt',
				},
				{
					displayName: 'Prompt',
					name: 'promptId',
					type: 'options',
					typeOptions: {
						loadOptionsMethod: 'getPrompts',
					},
					required: true,
					default: '',
					description: 'The prompt to use',
				},
				{
					displayName: 'Version Selection',
					name: 'versionSelector',
					type: 'options',
					options: [
						{
							name: 'Latest Version',
							value: 'latest',
						},
						{
							name: 'Specific Version',
							value: 'version',
						},
						{
							name: 'By Tag',
							value: 'tag',
						},
					],
					default: 'latest',
					displayOptions: {
						show: {
							operation: ['getPrompt'],
						},
					},
				},
				{
					displayName: 'Version',
					name: 'versionId',
					type: 'options',
					typeOptions: {
						loadOptionsMethod: 'getVersions',
						loadOptionsDependsOn: ['promptId'],
					},
					default: '',
					displayOptions: {
						show: {
							operation: ['getPrompt'],
							versionSelector: ['version'],
						},
					},
				},
				{
					displayName: 'Tag',
					name: 'tagName',
					type: 'options',
					typeOptions: {
						loadOptionsMethod: 'getTags',
						loadOptionsDependsOn: ['promptId'],
					},
					default: '',
					displayOptions: {
						show: {
							operation: ['getPrompt'],
							versionSelector: ['tag'],
						},
					},
				},
				{
					displayName: 'Context Variables',
					name: 'context',
					type: 'json',
					default: '={{ $json }}',
					description: 'Variables for template rendering',
					displayOptions: {
						show: {
							operation: ['getPrompt'],
						},
					},
				},
				{
					displayName: 'Version',
					name: 'feedbackVersionId',
					type: 'options',
					typeOptions: {
						loadOptionsMethod: 'getVersions',
						loadOptionsDependsOn: ['promptId'],
					},
					required: true,
					default: '',
					displayOptions: {
						show: {
							operation: ['submitFeedback'],
						},
					},
				},
				{
					displayName: 'Rating',
					name: 'rating',
					type: 'number',
					typeOptions: {
						minValue: 1,
						maxValue: 5,
						numberStepSize: 1,
					},
					required: true,
					default: 5,
					displayOptions: {
						show: {
							operation: ['submitFeedback'],
						},
					},
				},
				{
					displayName: 'Comment',
					name: 'comment',
					type: 'string',
					typeOptions: {
						rows: 4,
					},
					default: '',
					displayOptions: {
						show: {
							operation: ['submitFeedback'],
						},
					},
				},
				{
					displayName: 'Test Input',
					name: 'testInput',
					type: 'string',
					default: '',
					description: 'Input used in the test scenario',
					displayOptions: {
						show: {
							operation: ['submitFeedback'],
						},
					},
				},
				{
					displayName: 'Test Actual Output',
					name: 'testActualOutput',
					type: 'string',
					default: '',
					description: 'Actual output from the test',
					displayOptions: {
						show: {
							operation: ['submitFeedback'],
						},
					},
				},
				{
					displayName: 'Test Expected Output',
					name: 'testExpectedOutput',
					type: 'string',
					default: '',
					description: 'Expected output for the test',
					displayOptions: {
						show: {
							operation: ['submitFeedback'],
						},
					},
				},
			],
		};
	}

	methods = {
		loadOptions: {
			async getPrompts(this: ILoadOptionsFunctions): Promise<INodePropertyOptions[]> {
				const credentials = await this.getCredentials('promptKaizenApi');
				const baseUrl = (credentials.baseUrl as string).replace(/\/$/, '');
				const apiKey = credentials.apiKey as string;

				const response = await this.helpers.httpRequest({
					method: 'GET',
					url: `${baseUrl}/prompts`,
					headers: { 'x-api-key': apiKey },
					json: true,
				});

				return response.map((prompt: any) => ({
					name: `${prompt.name} (${prompt.prompt_type})`,
					value: prompt.id,
					description: prompt.description || '',
				}));
			},

			async getVersions(this: ILoadOptionsFunctions): Promise<INodePropertyOptions[]> {
				const promptId = this.getNodeParameter('promptId') as string;
				if (!promptId) return [];

				const credentials = await this.getCredentials('promptKaizenApi');
				const baseUrl = (credentials.baseUrl as string).replace(/\/$/, '');
				const apiKey = credentials.apiKey as string;

				const response = await this.helpers.httpRequest({
					method: 'GET',
					url: `${baseUrl}/prompts/${promptId}`,
					headers: { 'x-api-key': apiKey },
					json: true,
				});

				return response.versions.map((version: any) => ({
					name: `${version.version} (${version.content_type})`,
					value: version.id,
					description: version.changelog || 'No changelog',
				}));
			},

			async getTags(this: ILoadOptionsFunctions): Promise<INodePropertyOptions[]> {
				const promptId = this.getNodeParameter('promptId') as string;
				if (!promptId) return [];

				const credentials = await this.getCredentials('promptKaizenApi');
				const baseUrl = (credentials.baseUrl as string).replace(/\/$/, '');
				const apiKey = credentials.apiKey as string;

				const response = await this.helpers.httpRequest({
					method: 'GET',
					url: `${baseUrl}/prompts/${promptId}`,
					headers: { 'x-api-key': apiKey },
					json: true,
				});

				return response.tags.map((tag: any) => ({
					name: tag.name,
					value: tag.name,
				}));
			},
		},
	};

	async execute(this: IExecuteFunctions): Promise<INodeExecutionData[][]> {
		const items = this.getInputData();
		const returnData: INodeExecutionData[] = [];
		const credentials = await this.getCredentials('promptKaizenApi');
		const baseUrl = (credentials.baseUrl as string).replace(/\/$/, '');
		const apiKey = credentials.apiKey as string;

		for (let i = 0; i < items.length; i++) {
			const operation = this.getNodeParameter('operation', i) as string;
			const promptId = this.getNodeParameter('promptId', i) as string;

			let responseData: any;

			if (operation === 'getPrompt') {
				const versionSelector = this.getNodeParameter('versionSelector', i) as string;
				const contextStr = this.getNodeParameter('context', i, '{}') as string;
				let context: any = {};

				try {
					context = typeof contextStr === 'string' ? JSON.parse(contextStr) : contextStr;
				} catch {
					context = {};
				}

				let versionData: any;
				let promptData: any;

				if (versionSelector === 'latest') {
					promptData = await this.helpers.httpRequest({
						method: 'GET',
						url: `${baseUrl}/prompts/${promptId}`,
						headers: { 'x-api-key': apiKey },
						json: true,
					});
					versionData = promptData.versions[promptData.versions.length - 1];
				} else if (versionSelector === 'version') {
					const versionId = this.getNodeParameter('versionId', i) as string;
					versionData = await this.helpers.httpRequest({
						method: 'GET',
						url: `${baseUrl}/prompts/${promptId}/versions/${versionId}`,
						headers: { 'x-api-key': apiKey },
						json: true,
					});
					promptData = await this.helpers.httpRequest({
						method: 'GET',
						url: `${baseUrl}/prompts/${promptId}`,
						headers: { 'x-api-key': apiKey },
						json: true,
					});
				} else {
					const tagName = this.getNodeParameter('tagName', i) as string;
					versionData = await this.helpers.httpRequest({
						method: 'GET',
						url: `${baseUrl}/prompts/${promptId}/tags/${tagName}/version`,
						headers: { 'x-api-key': apiKey },
						json: true,
					});
					promptData = await this.helpers.httpRequest({
						method: 'GET',
						url: `${baseUrl}/prompts/${promptId}`,
						headers: { 'x-api-key': apiKey },
						json: true,
					});
				}

				let content = versionData.content;

				if (versionData.content_type === 'template' && Object.keys(context).length > 0) {
					const renderResponse = await this.helpers.httpRequest({
						method: 'POST',
						url: `${baseUrl}/prompts/${promptId}/versions/${versionData.id}/render`,
						headers: { 'x-api-key': apiKey },
						body: { context },
						json: true,
					});
					content = renderResponse.rendered_content;
				}

				responseData = {
					prompt_id: promptId,
					version_id: versionData.id,
					version: versionData.version,
					content_type: versionData.content_type,
					prompt_type: promptData.prompt_type,
					content,
					raw_content: versionData.content,
					variables: versionData.variables,
					context_used: context,
				};

			} else if (operation === 'submitFeedback') {
				const versionId = this.getNodeParameter('feedbackVersionId', i) as string;
				const rating = this.getNodeParameter('rating', i) as number;
				const comment = this.getNodeParameter('comment', i, '') as string;
				const testInput = this.getNodeParameter('testInput', i, '') as string;
				const testActualOutput = this.getNodeParameter('testActualOutput', i, '') as string;
				const testExpectedOutput = this.getNodeParameter('testExpectedOutput', i, '') as string;

				const body: any = {
					version_id: versionId,
					rating,
				};

				if (comment) body.comment = comment;
				if (testInput) body.test_input = testInput;
				if (testActualOutput) body.test_actual_output = testActualOutput;
				if (testExpectedOutput) body.test_expected_output = testExpectedOutput;

				responseData = await this.helpers.httpRequest({
					method: 'POST',
					url: `${baseUrl}/prompts/${promptId}/feedback`,
					headers: { 'x-api-key': apiKey },
					body,
					json: true,
				});
			}

			returnData.push({ json: responseData });
		}

		return [returnData];
	}
}