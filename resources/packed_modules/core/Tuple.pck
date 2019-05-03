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
            "Var": [
              [
                1163,
                1164
              ],
              "a"
            ]
          },
          {
            "Var": [
              [
                1165,
                1166
              ],
              "b"
            ]
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
              [
                1323,
                1328
              ],
              [
                {
                  "Var": [
                    [
                      1324,
                      1325
                    ],
                    "x"
                  ]
                },
                {
                  "Wildcard": [
                    1326,
                    1327
                  ]
                }
              ]
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
              [
                1471,
                1476
              ],
              [
                {
                  "Wildcard": [
                    1472,
                    1473
                  ]
                },
                {
                  "Var": [
                    [
                      1474,
                      1475
                    ],
                    "y"
                  ]
                }
              ]
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
            "Var": [
              [
                1730,
                1734
              ],
              "func"
            ]
          },
          {
            "Tuple": [
              [
                1735,
                1740
              ],
              [
                {
                  "Var": [
                    [
                      1736,
                      1737
                    ],
                    "x"
                  ]
                },
                {
                  "Var": [
                    [
                      1738,
                      1739
                    ],
                    "y"
                  ]
                }
              ]
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
            "Var": [
              [
                1973,
                1977
              ],
              "func"
            ]
          },
          {
            "Tuple": [
              [
                1978,
                1983
              ],
              [
                {
                  "Var": [
                    [
                      1979,
                      1980
                    ],
                    "x"
                  ]
                },
                {
                  "Var": [
                    [
                      1981,
                      1982
                    ],
                    "y"
                  ]
                }
              ]
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
            "Var": [
              [
                2252,
                2257
              ],
              "funcA"
            ]
          },
          {
            "Var": [
              [
                2258,
                2263
              ],
              "funcB"
            ]
          },
          {
            "Tuple": [
              [
                2264,
                2269
              ],
              [
                {
                  "Var": [
                    [
                      2265,
                      2266
                    ],
                    "x"
                  ]
                },
                {
                  "Var": [
                    [
                      2267,
                      2268
                    ],
                    "y"
                  ]
                }
              ]
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