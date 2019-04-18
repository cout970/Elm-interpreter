{
  "header": {
    "name": "Tuple",
    "exposing": {
      "Just": [
        {
          "Definition": "pair"
        },
        {
          "Definition": "first"
        },
        {
          "Definition": "second"
        },
        {
          "Definition": "mapFirst"
        },
        {
          "Definition": "mapSecond"
        },
        {
          "Definition": "mapBoth"
        }
      ]
    }
  },
  "imports": [],
  "statements": [
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Var": "a"
            },
            {
              "Fun": [
                {
                  "Var": "b"
                },
                {
                  "Tuple": [
                    {
                      "Var": "a"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "pair",
        "patterns": [
          {
            "Var": "a"
          },
          {
            "Var": "b"
          }
        ],
        "expr": {
          "Tuple": [
            [
              1171,
              1177
            ],
            [
              {
                "Ref": [
                  [
                    1172,
                    1173
                  ],
                  "a"
                ]
              },
              {
                "Ref": [
                  [
                    1175,
                    1176
                  ],
                  "b"
                ]
              }
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tuple": [
                {
                  "Var": "a"
                },
                {
                  "Var": "b"
                }
              ]
            },
            {
              "Var": "a"
            }
          ]
        },
        "name": "first",
        "patterns": [
          {
            "Tuple": [
              {
                "Var": "x"
              },
              "Wildcard"
            ]
          }
        ],
        "expr": {
          "Ref": [
            [
              1333,
              1334
            ],
            "x"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Tuple": [
                {
                  "Var": "a"
                },
                {
                  "Var": "b"
                }
              ]
            },
            {
              "Var": "b"
            }
          ]
        },
        "name": "second",
        "patterns": [
          {
            "Tuple": [
              "Wildcard",
              {
                "Var": "y"
              }
            ]
          }
        ],
        "expr": {
          "Ref": [
            [
              1481,
              1482
            ],
            "y"
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Var": "x"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tuple": [
                    {
                      "Var": "a"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                },
                {
                  "Tuple": [
                    {
                      "Var": "x"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "mapFirst",
        "patterns": [
          {
            "Var": "func"
          },
          {
            "Tuple": [
              {
                "Var": "x"
              },
              {
                "Var": "y"
              }
            ]
          }
        ],
        "expr": {
          "Tuple": [
            [
              1745,
              1756
            ],
            [
              {
                "Application": [
                  [
                    1746,
                    1752
                  ],
                  {
                    "Ref": [
                      [
                        1746,
                        1750
                      ],
                      "func"
                    ]
                  },
                  {
                    "Ref": [
                      [
                        1751,
                        1752
                      ],
                      "x"
                    ]
                  }
                ]
              },
              {
                "Ref": [
                  [
                    1754,
                    1755
                  ],
                  "y"
                ]
              }
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "b"
                },
                {
                  "Var": "y"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tuple": [
                    {
                      "Var": "a"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                },
                {
                  "Tuple": [
                    {
                      "Var": "a"
                    },
                    {
                      "Var": "y"
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "mapSecond",
        "patterns": [
          {
            "Var": "func"
          },
          {
            "Tuple": [
              {
                "Var": "x"
              },
              {
                "Var": "y"
              }
            ]
          }
        ],
        "expr": {
          "Tuple": [
            [
              1988,
              1999
            ],
            [
              {
                "Ref": [
                  [
                    1989,
                    1990
                  ],
                  "x"
                ]
              },
              {
                "Application": [
                  [
                    1992,
                    1998
                  ],
                  {
                    "Ref": [
                      [
                        1992,
                        1996
                      ],
                      "func"
                    ]
                  },
                  {
                    "Ref": [
                      [
                        1997,
                        1998
                      ],
                      "y"
                    ]
                  }
                ]
              }
            ]
          ]
        }
      }
    },
    {
      "Def": {
        "header": {
          "Fun": [
            {
              "Fun": [
                {
                  "Var": "a"
                },
                {
                  "Var": "x"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Var": "y"
                    }
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tuple": [
                        {
                          "Var": "a"
                        },
                        {
                          "Var": "b"
                        }
                      ]
                    },
                    {
                      "Tuple": [
                        {
                          "Var": "x"
                        },
                        {
                          "Var": "y"
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "mapBoth",
        "patterns": [
          {
            "Var": "funcA"
          },
          {
            "Var": "funcB"
          },
          {
            "Tuple": [
              {
                "Var": "x"
              },
              {
                "Var": "y"
              }
            ]
          }
        ],
        "expr": {
          "Tuple": [
            [
              2274,
              2294
            ],
            [
              {
                "Application": [
                  [
                    2276,
                    2283
                  ],
                  {
                    "Ref": [
                      [
                        2276,
                        2281
                      ],
                      "funcA"
                    ]
                  },
                  {
                    "Ref": [
                      [
                        2282,
                        2283
                      ],
                      "x"
                    ]
                  }
                ]
              },
              {
                "Application": [
                  [
                    2285,
                    2292
                  ],
                  {
                    "Ref": [
                      [
                        2285,
                        2290
                      ],
                      "funcB"
                    ]
                  },
                  {
                    "Ref": [
                      [
                        2291,
                        2292
                      ],
                      "y"
                    ]
                  }
                ]
              }
            ]
          ]
        }
      }
    }
  ]
}