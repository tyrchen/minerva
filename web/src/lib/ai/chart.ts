import { BaseAssistant } from './index';
import { OpenAI } from 'openai';

export class ChartAsistant extends BaseAssistant {
  getInstructions() {
    return [
      {
        role: 'system',
        content:
          'You are statistical data analyzer, you help in extract and visualize statistical data from paragraph of text',
      },
      {
        role: 'user',
        content: `I'll provide you with a text that may contain statistical data.
            If the text does not contain statistical data, simply response with 'No statistical data'.
            If there is statistical data, please extract them as a table (in markdown format) delimited by 4 equal marks ====.
            After that, please provide a YAML structure that represents a config for Chart.js library based on the following JSON schema:
            ${JSON.stringify(this.getJSONSchema())}

            The YAML structure must be delimited by \`\`\`.
            After that list important trends in the extracted data if any.
            So, the expected response for the first response and subsequence responses for modifications will be:
            ====
            {Table}
            ====
            \`\`\`
            {YAML}
            \`\`\`
            {Trends}
            `,
      },
      { role: 'assistant', content: 'Sure, please provide your text' },
    ] as OpenAI.ChatCompletionMessageParam[];
  }

  getJSONSchema() {
    return {
      $schema: 'http://json-schema.org/draft-07/schema#',
      title: 'ChartConfig',
      type: 'object',
      properties: {
        type: {
          type: 'string',
          enum: ['line', 'bar', 'pie', 'doughnut', 'radar', 'polarArea', 'bubble', 'scatter'],
        },
        data: {
          type: 'object',
          properties: {
            labels: {
              type: 'array',
              items: {
                type: 'string',
              },
            },
            datasets: {
              type: 'array',
              items: {
                type: 'object',
                properties: {
                  label: {
                    type: 'string',
                  },
                  data: {
                    type: 'array',
                    items: {
                      type: 'number',
                    },
                  },
                  backgroundColor: {
                    type: 'string',
                  },
                  borderColor: {
                    type: 'string',
                  },
                  borderWidth: {
                    type: 'number',
                  },
                },
                required: ['label', 'data'],
              },
            },
          },
          required: ['labels', 'datasets'],
        },
        options: {
          type: 'object',
          properties: {
            title: {
              type: 'object',
              properties: {
                display: {
                  type: 'boolean',
                },
                text: {
                  type: 'string',
                },
              },
            },
            legend: {
              type: 'object',
              properties: {
                display: {
                  type: 'boolean',
                },
                position: {
                  type: 'string',
                  enum: ['top', 'bottom', 'left', 'right'],
                },
              },
            },
            scales: {
              type: 'object',
              properties: {
                xAxes: {
                  type: 'array',
                  items: {
                    type: 'object',
                    properties: {
                      scaleLabel: {
                        type: 'object',
                        properties: {
                          display: {
                            type: 'boolean',
                          },
                          labelString: {
                            type: 'string',
                          },
                        },
                      },
                    },
                  },
                },
                yAxes: {
                  type: 'array',
                  items: {
                    type: 'object',
                    properties: {
                      ticks: {
                        type: 'object',
                        properties: {
                          beginAtZero: {
                            type: 'boolean',
                          },
                        },
                      },
                      scaleLabel: {
                        type: 'object',
                        properties: {
                          display: {
                            type: 'boolean',
                          },
                          labelString: {
                            type: 'string',
                          },
                        },
                      },
                    },
                  },
                },
              },
            },
          },
        },
      },
      required: ['type', 'data'],
    };
  }
}
