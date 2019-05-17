{
  "header": {
    "name": "Maybe",
    "exposing": {
      "Just": [
        {
          "Adt": [
            "Maybe",
            "All"
          ]
        },
        {
          "Definition": "andThen"
        },
        {
          "Definition": "map"
        },
        {
          "Definition": "map2"
        },
        {
          "Definition": "map3"
        },
        {
          "Definition": "map4"
        },
        {
          "Definition": "map5"
        },
        {
          "Definition": "withDefault"
        }
      ]
    }
  },
  "imports": [
    {
      "path": [
        "Basics"
      ],
      "alias": null,
      "exposing": {
        "Just": [
          {
            "Adt": [
              "Bool",
              "All"
            ]
          }
        ]
      }
    }
  ],
  "statements": [
    {
      "Adt": [
        "Maybe",
        [
          "a"
        ],
        [
          [
            [
              870,
              876
            ],
            "Just",
            [
              {
                "Var": "a"
              }
            ]
          ],
          [
            [
              883,
              890
            ],
            "Nothing",
            []
          ]
        ]
      ]
    },
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
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Var": "a"
                }
              ]
            }
          ]
        },
        "name": "withDefault",
        "patterns": [
          {
            "Var": [
              [
                1542,
                1549
              ],
              "default"
            ]
          },
          {
            "Var": [
              [
                1550,
                1555
              ],
              "maybe"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              1562,
              1626
            ],
            {
              "Ref": [
                [
                  1567,
                  1572
                ],
                "maybe"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      1582,
                      1592
                    ],
                    "Just",
                    [
                      {
                        "Var": [
                          [
                            1587,
                            1592
                          ],
                          "value"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Ref": [
                    [
                      1596,
                      1601
                    ],
                    "value"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      1608,
                      1615
                    ],
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      1619,
                      1626
                    ],
                    "default"
                  ]
                }
              ]
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
                  "Var": "b"
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "map",
        "patterns": [
          {
            "Var": [
              [
                1883,
                1884
              ],
              "f"
            ]
          },
          {
            "Var": [
              [
                1885,
                1890
              ],
              "maybe"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              1895,
              1977
            ],
            {
              "Ref": [
                [
                  1900,
                  1905
                ],
                "maybe"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      1913,
                      1923
                    ],
                    "Just",
                    [
                      {
                        "Var": [
                          [
                            1918,
                            1923
                          ],
                          "value"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      1933,
                      1947
                    ],
                    {
                      "Ref": [
                        [
                          1933,
                          1938
                        ],
                        "Just"
                      ]
                    },
                    {
                      "Application": [
                        [
                          1939,
                          1946
                        ],
                        {
                          "Ref": [
                            [
                              1939,
                              1940
                            ],
                            "f"
                          ]
                        },
                        {
                          "Ref": [
                            [
                              1941,
                              1946
                            ],
                            "value"
                          ]
                        }
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      1953,
                      1960
                    ],
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      1970,
                      1977
                    ],
                    "Nothing"
                  ]
                }
              ]
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
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Var": "value"
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "value"
                          }
                        ]
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map2",
        "patterns": [
          {
            "Var": [
              [
                2430,
                2434
              ],
              "func"
            ]
          },
          {
            "Var": [
              [
                2435,
                2437
              ],
              "ma"
            ]
          },
          {
            "Var": [
              [
                2438,
                2440
              ],
              "mb"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              2445,
              2598
            ],
            {
              "Ref": [
                [
                  2450,
                  2452
                ],
                "ma"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      2460,
                      2467
                    ],
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      2477,
                      2484
                    ],
                    "Nothing"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      2490,
                      2496
                    ],
                    "Just",
                    [
                      {
                        "Var": [
                          [
                            2495,
                            2496
                          ],
                          "a"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      2506,
                      2598
                    ],
                    {
                      "Ref": [
                        [
                          2511,
                          2513
                        ],
                        "mb"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            [
                              2525,
                              2532
                            ],
                            "Nothing",
                            []
                          ]
                        },
                        {
                          "Ref": [
                            [
                              2546,
                              2553
                            ],
                            "Nothing"
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            [
                              2563,
                              2569
                            ],
                            "Just",
                            [
                              {
                                "Var": [
                                  [
                                    2568,
                                    2569
                                  ],
                                  "b"
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Application": [
                            [
                              2583,
                              2598
                            ],
                            {
                              "Ref": [
                                [
                                  2583,
                                  2588
                                ],
                                "Just"
                              ]
                            },
                            {
                              "Application": [
                                [
                                  2589,
                                  2597
                                ],
                                {
                                  "Application": [
                                    [
                                      2589,
                                      2597
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          2589,
                                          2593
                                        ],
                                        "func"
                                      ]
                                    },
                                    {
                                      "Ref": [
                                        [
                                          2594,
                                          2595
                                        ],
                                        "a"
                                      ]
                                    }
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      2596,
                                      2597
                                    ],
                                    "b"
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
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
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Var": "value"
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "Maybe",
                            [
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Tag": [
                            "Maybe",
                            [
                              {
                                "Var": "value"
                              }
                            ]
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map3",
        "patterns": [
          {
            "Var": [
              [
                2690,
                2694
              ],
              "func"
            ]
          },
          {
            "Var": [
              [
                2695,
                2697
              ],
              "ma"
            ]
          },
          {
            "Var": [
              [
                2698,
                2700
              ],
              "mb"
            ]
          },
          {
            "Var": [
              [
                2701,
                2703
              ],
              "mc"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              2708,
              2956
            ],
            {
              "Ref": [
                [
                  2713,
                  2715
                ],
                "ma"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      2723,
                      2730
                    ],
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      2740,
                      2747
                    ],
                    "Nothing"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      2753,
                      2759
                    ],
                    "Just",
                    [
                      {
                        "Var": [
                          [
                            2758,
                            2759
                          ],
                          "a"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      2769,
                      2956
                    ],
                    {
                      "Ref": [
                        [
                          2774,
                          2776
                        ],
                        "mb"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            [
                              2788,
                              2795
                            ],
                            "Nothing",
                            []
                          ]
                        },
                        {
                          "Ref": [
                            [
                              2809,
                              2816
                            ],
                            "Nothing"
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            [
                              2826,
                              2832
                            ],
                            "Just",
                            [
                              {
                                "Var": [
                                  [
                                    2831,
                                    2832
                                  ],
                                  "b"
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Case": [
                            [
                              2846,
                              2956
                            ],
                            {
                              "Ref": [
                                [
                                  2851,
                                  2853
                                ],
                                "mc"
                              ]
                            },
                            [
                              [
                                {
                                  "Adt": [
                                    [
                                      2869,
                                      2876
                                    ],
                                    "Nothing",
                                    []
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      2894,
                                      2901
                                    ],
                                    "Nothing"
                                  ]
                                }
                              ],
                              [
                                {
                                  "Adt": [
                                    [
                                      2915,
                                      2921
                                    ],
                                    "Just",
                                    [
                                      {
                                        "Var": [
                                          [
                                            2920,
                                            2921
                                          ],
                                          "c"
                                        ]
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Application": [
                                    [
                                      2939,
                                      2956
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          2939,
                                          2944
                                        ],
                                        "Just"
                                      ]
                                    },
                                    {
                                      "Application": [
                                        [
                                          2945,
                                          2955
                                        ],
                                        {
                                          "Application": [
                                            [
                                              2945,
                                              2955
                                            ],
                                            {
                                              "Application": [
                                                [
                                                  2945,
                                                  2955
                                                ],
                                                {
                                                  "Ref": [
                                                    [
                                                      2945,
                                                      2949
                                                    ],
                                                    "func"
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      2950,
                                                      2951
                                                    ],
                                                    "a"
                                                  ]
                                                }
                                              ]
                                            },
                                            {
                                              "Ref": [
                                                [
                                                  2952,
                                                  2953
                                                ],
                                                "b"
                                              ]
                                            }
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              2954,
                                              2955
                                            ],
                                            "c"
                                          ]
                                        }
                                      ]
                                    }
                                  ]
                                }
                              ]
                            ]
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
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
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Fun": [
                            {
                              "Var": "d"
                            },
                            {
                              "Var": "value"
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "Maybe",
                            [
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Fun": [
                            {
                              "Tag": [
                                "Maybe",
                                [
                                  {
                                    "Var": "d"
                                  }
                                ]
                              ]
                            },
                            {
                              "Tag": [
                                "Maybe",
                                [
                                  {
                                    "Var": "value"
                                  }
                                ]
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map4",
        "patterns": [
          {
            "Var": [
              [
                3064,
                3068
              ],
              "func"
            ]
          },
          {
            "Var": [
              [
                3069,
                3071
              ],
              "ma"
            ]
          },
          {
            "Var": [
              [
                3072,
                3074
              ],
              "mb"
            ]
          },
          {
            "Var": [
              [
                3075,
                3077
              ],
              "mc"
            ]
          },
          {
            "Var": [
              [
                3078,
                3080
              ],
              "md"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              3085,
              3444
            ],
            {
              "Ref": [
                [
                  3090,
                  3092
                ],
                "ma"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      3100,
                      3107
                    ],
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      3117,
                      3124
                    ],
                    "Nothing"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      3130,
                      3136
                    ],
                    "Just",
                    [
                      {
                        "Var": [
                          [
                            3135,
                            3136
                          ],
                          "a"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      3146,
                      3444
                    ],
                    {
                      "Ref": [
                        [
                          3151,
                          3153
                        ],
                        "mb"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            [
                              3165,
                              3172
                            ],
                            "Nothing",
                            []
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3186,
                              3193
                            ],
                            "Nothing"
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            [
                              3203,
                              3209
                            ],
                            "Just",
                            [
                              {
                                "Var": [
                                  [
                                    3208,
                                    3209
                                  ],
                                  "b"
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Case": [
                            [
                              3223,
                              3444
                            ],
                            {
                              "Ref": [
                                [
                                  3228,
                                  3230
                                ],
                                "mc"
                              ]
                            },
                            [
                              [
                                {
                                  "Adt": [
                                    [
                                      3246,
                                      3253
                                    ],
                                    "Nothing",
                                    []
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      3271,
                                      3278
                                    ],
                                    "Nothing"
                                  ]
                                }
                              ],
                              [
                                {
                                  "Adt": [
                                    [
                                      3292,
                                      3298
                                    ],
                                    "Just",
                                    [
                                      {
                                        "Var": [
                                          [
                                            3297,
                                            3298
                                          ],
                                          "c"
                                        ]
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Case": [
                                    [
                                      3316,
                                      3444
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          3321,
                                          3323
                                        ],
                                        "md"
                                      ]
                                    },
                                    [
                                      [
                                        {
                                          "Adt": [
                                            [
                                              3343,
                                              3350
                                            ],
                                            "Nothing",
                                            []
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3372,
                                              3379
                                            ],
                                            "Nothing"
                                          ]
                                        }
                                      ],
                                      [
                                        {
                                          "Adt": [
                                            [
                                              3397,
                                              3403
                                            ],
                                            "Just",
                                            [
                                              {
                                                "Var": [
                                                  [
                                                    3402,
                                                    3403
                                                  ],
                                                  "d"
                                                ]
                                              }
                                            ]
                                          ]
                                        },
                                        {
                                          "Application": [
                                            [
                                              3425,
                                              3444
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  3425,
                                                  3430
                                                ],
                                                "Just"
                                              ]
                                            },
                                            {
                                              "Application": [
                                                [
                                                  3431,
                                                  3443
                                                ],
                                                {
                                                  "Application": [
                                                    [
                                                      3431,
                                                      3443
                                                    ],
                                                    {
                                                      "Application": [
                                                        [
                                                          3431,
                                                          3443
                                                        ],
                                                        {
                                                          "Application": [
                                                            [
                                                              3431,
                                                              3443
                                                            ],
                                                            {
                                                              "Ref": [
                                                                [
                                                                  3431,
                                                                  3435
                                                                ],
                                                                "func"
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  3436,
                                                                  3437
                                                                ],
                                                                "a"
                                                              ]
                                                            }
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              3438,
                                                              3439
                                                            ],
                                                            "b"
                                                          ]
                                                        }
                                                      ]
                                                    },
                                                    {
                                                      "Ref": [
                                                        [
                                                          3440,
                                                          3441
                                                        ],
                                                        "c"
                                                      ]
                                                    }
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      3442,
                                                      3443
                                                    ],
                                                    "d"
                                                  ]
                                                }
                                              ]
                                            }
                                          ]
                                        }
                                      ]
                                    ]
                                  ]
                                }
                              ]
                            ]
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
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
                  "Fun": [
                    {
                      "Var": "b"
                    },
                    {
                      "Fun": [
                        {
                          "Var": "c"
                        },
                        {
                          "Fun": [
                            {
                              "Var": "d"
                            },
                            {
                              "Fun": [
                                {
                                  "Var": "e"
                                },
                                {
                                  "Var": "value"
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "b"
                          }
                        ]
                      ]
                    },
                    {
                      "Fun": [
                        {
                          "Tag": [
                            "Maybe",
                            [
                              {
                                "Var": "c"
                              }
                            ]
                          ]
                        },
                        {
                          "Fun": [
                            {
                              "Tag": [
                                "Maybe",
                                [
                                  {
                                    "Var": "d"
                                  }
                                ]
                              ]
                            },
                            {
                              "Fun": [
                                {
                                  "Tag": [
                                    "Maybe",
                                    [
                                      {
                                        "Var": "e"
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Tag": [
                                    "Maybe",
                                    [
                                      {
                                        "Var": "value"
                                      }
                                    ]
                                  ]
                                }
                              ]
                            }
                          ]
                        }
                      ]
                    }
                  ]
                }
              ]
            }
          ]
        },
        "name": "map5",
        "patterns": [
          {
            "Var": [
              [
                3568,
                3572
              ],
              "func"
            ]
          },
          {
            "Var": [
              [
                3573,
                3575
              ],
              "ma"
            ]
          },
          {
            "Var": [
              [
                3576,
                3578
              ],
              "mb"
            ]
          },
          {
            "Var": [
              [
                3579,
                3581
              ],
              "mc"
            ]
          },
          {
            "Var": [
              [
                3582,
                3584
              ],
              "md"
            ]
          },
          {
            "Var": [
              [
                3585,
                3587
              ],
              "me"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              3592,
              4078
            ],
            {
              "Ref": [
                [
                  3597,
                  3599
                ],
                "ma"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      3607,
                      3614
                    ],
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      3624,
                      3631
                    ],
                    "Nothing"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      3637,
                      3643
                    ],
                    "Just",
                    [
                      {
                        "Var": [
                          [
                            3642,
                            3643
                          ],
                          "a"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Case": [
                    [
                      3653,
                      4078
                    ],
                    {
                      "Ref": [
                        [
                          3658,
                          3660
                        ],
                        "mb"
                      ]
                    },
                    [
                      [
                        {
                          "Adt": [
                            [
                              3672,
                              3679
                            ],
                            "Nothing",
                            []
                          ]
                        },
                        {
                          "Ref": [
                            [
                              3693,
                              3700
                            ],
                            "Nothing"
                          ]
                        }
                      ],
                      [
                        {
                          "Adt": [
                            [
                              3710,
                              3716
                            ],
                            "Just",
                            [
                              {
                                "Var": [
                                  [
                                    3715,
                                    3716
                                  ],
                                  "b"
                                ]
                              }
                            ]
                          ]
                        },
                        {
                          "Case": [
                            [
                              3730,
                              4078
                            ],
                            {
                              "Ref": [
                                [
                                  3735,
                                  3737
                                ],
                                "mc"
                              ]
                            },
                            [
                              [
                                {
                                  "Adt": [
                                    [
                                      3753,
                                      3760
                                    ],
                                    "Nothing",
                                    []
                                  ]
                                },
                                {
                                  "Ref": [
                                    [
                                      3778,
                                      3785
                                    ],
                                    "Nothing"
                                  ]
                                }
                              ],
                              [
                                {
                                  "Adt": [
                                    [
                                      3799,
                                      3805
                                    ],
                                    "Just",
                                    [
                                      {
                                        "Var": [
                                          [
                                            3804,
                                            3805
                                          ],
                                          "c"
                                        ]
                                      }
                                    ]
                                  ]
                                },
                                {
                                  "Case": [
                                    [
                                      3823,
                                      4078
                                    ],
                                    {
                                      "Ref": [
                                        [
                                          3828,
                                          3830
                                        ],
                                        "md"
                                      ]
                                    },
                                    [
                                      [
                                        {
                                          "Adt": [
                                            [
                                              3850,
                                              3857
                                            ],
                                            "Nothing",
                                            []
                                          ]
                                        },
                                        {
                                          "Ref": [
                                            [
                                              3879,
                                              3886
                                            ],
                                            "Nothing"
                                          ]
                                        }
                                      ],
                                      [
                                        {
                                          "Adt": [
                                            [
                                              3904,
                                              3910
                                            ],
                                            "Just",
                                            [
                                              {
                                                "Var": [
                                                  [
                                                    3909,
                                                    3910
                                                  ],
                                                  "d"
                                                ]
                                              }
                                            ]
                                          ]
                                        },
                                        {
                                          "Case": [
                                            [
                                              3932,
                                              4078
                                            ],
                                            {
                                              "Ref": [
                                                [
                                                  3937,
                                                  3939
                                                ],
                                                "me"
                                              ]
                                            },
                                            [
                                              [
                                                {
                                                  "Adt": [
                                                    [
                                                      3963,
                                                      3970
                                                    ],
                                                    "Nothing",
                                                    []
                                                  ]
                                                },
                                                {
                                                  "Ref": [
                                                    [
                                                      3996,
                                                      4003
                                                    ],
                                                    "Nothing"
                                                  ]
                                                }
                                              ],
                                              [
                                                {
                                                  "Adt": [
                                                    [
                                                      4025,
                                                      4031
                                                    ],
                                                    "Just",
                                                    [
                                                      {
                                                        "Var": [
                                                          [
                                                            4030,
                                                            4031
                                                          ],
                                                          "e"
                                                        ]
                                                      }
                                                    ]
                                                  ]
                                                },
                                                {
                                                  "Application": [
                                                    [
                                                      4057,
                                                      4078
                                                    ],
                                                    {
                                                      "Ref": [
                                                        [
                                                          4057,
                                                          4062
                                                        ],
                                                        "Just"
                                                      ]
                                                    },
                                                    {
                                                      "Application": [
                                                        [
                                                          4063,
                                                          4077
                                                        ],
                                                        {
                                                          "Application": [
                                                            [
                                                              4063,
                                                              4077
                                                            ],
                                                            {
                                                              "Application": [
                                                                [
                                                                  4063,
                                                                  4077
                                                                ],
                                                                {
                                                                  "Application": [
                                                                    [
                                                                      4063,
                                                                      4077
                                                                    ],
                                                                    {
                                                                      "Application": [
                                                                        [
                                                                          4063,
                                                                          4077
                                                                        ],
                                                                        {
                                                                          "Ref": [
                                                                            [
                                                                              4063,
                                                                              4067
                                                                            ],
                                                                            "func"
                                                                          ]
                                                                        },
                                                                        {
                                                                          "Ref": [
                                                                            [
                                                                              4068,
                                                                              4069
                                                                            ],
                                                                            "a"
                                                                          ]
                                                                        }
                                                                      ]
                                                                    },
                                                                    {
                                                                      "Ref": [
                                                                        [
                                                                          4070,
                                                                          4071
                                                                        ],
                                                                        "b"
                                                                      ]
                                                                    }
                                                                  ]
                                                                },
                                                                {
                                                                  "Ref": [
                                                                    [
                                                                      4072,
                                                                      4073
                                                                    ],
                                                                    "c"
                                                                  ]
                                                                }
                                                              ]
                                                            },
                                                            {
                                                              "Ref": [
                                                                [
                                                                  4074,
                                                                  4075
                                                                ],
                                                                "d"
                                                              ]
                                                            }
                                                          ]
                                                        },
                                                        {
                                                          "Ref": [
                                                            [
                                                              4076,
                                                              4077
                                                            ],
                                                            "e"
                                                          ]
                                                        }
                                                      ]
                                                    }
                                                  ]
                                                }
                                              ]
                                            ]
                                          ]
                                        }
                                      ]
                                    ]
                                  ]
                                }
                              ]
                            ]
                          ]
                        }
                      ]
                    ]
                  ]
                }
              ]
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
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            },
            {
              "Fun": [
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "a"
                      }
                    ]
                  ]
                },
                {
                  "Tag": [
                    "Maybe",
                    [
                      {
                        "Var": "b"
                      }
                    ]
                  ]
                }
              ]
            }
          ]
        },
        "name": "andThen",
        "patterns": [
          {
            "Var": [
              [
                5149,
                5157
              ],
              "callback"
            ]
          },
          {
            "Var": [
              [
                5158,
                5168
              ],
              "maybeValue"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              5175,
              5282
            ],
            {
              "Ref": [
                [
                  5180,
                  5190
                ],
                "maybeValue"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      5202,
                      5212
                    ],
                    "Just",
                    [
                      {
                        "Var": [
                          [
                            5207,
                            5212
                          ],
                          "value"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      5228,
                      5242
                    ],
                    {
                      "Ref": [
                        [
                          5228,
                          5236
                        ],
                        "callback"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5237,
                          5242
                        ],
                        "value"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      5252,
                      5259
                    ],
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      5275,
                      5282
                    ],
                    "Nothing"
                  ]
                }
              ]
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
              "Tag": [
                "Maybe",
                [
                  {
                    "Var": "a"
                  }
                ]
              ]
            },
            {
              "Tag": [
                "Bool",
                []
              ]
            }
          ]
        },
        "name": "isJust",
        "patterns": [
          {
            "Var": [
              [
                5396,
                5401
              ],
              "maybe"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              5406,
              5472
            ],
            {
              "Ref": [
                [
                  5411,
                  5416
                ],
                "maybe"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      5424,
                      5430
                    ],
                    "Just",
                    [
                      {
                        "Wildcard": [
                          5429,
                          5430
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Ref": [
                    [
                      5440,
                      5444
                    ],
                    "True"
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      5450,
                      5457
                    ],
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      5467,
                      5472
                    ],
                    "False"
                  ]
                }
              ]
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
              "Var": "b"
            },
            {
              "Fun": [
                {
                  "Fun": [
                    {
                      "Var": "a"
                    },
                    {
                      "Var": "b"
                    }
                  ]
                },
                {
                  "Fun": [
                    {
                      "Tag": [
                        "Maybe",
                        [
                          {
                            "Var": "a"
                          }
                        ]
                      ]
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
        "name": "destruct",
        "patterns": [
          {
            "Var": [
              [
                5525,
                5532
              ],
              "default"
            ]
          },
          {
            "Var": [
              [
                5533,
                5537
              ],
              "func"
            ]
          },
          {
            "Var": [
              [
                5538,
                5543
              ],
              "maybe"
            ]
          }
        ],
        "expr": {
          "Case": [
            [
              5548,
              5618
            ],
            {
              "Ref": [
                [
                  5553,
                  5558
                ],
                "maybe"
              ]
            },
            [
              [
                {
                  "Adt": [
                    [
                      5566,
                      5572
                    ],
                    "Just",
                    [
                      {
                        "Var": [
                          [
                            5571,
                            5572
                          ],
                          "a"
                        ]
                      }
                    ]
                  ]
                },
                {
                  "Application": [
                    [
                      5582,
                      5588
                    ],
                    {
                      "Ref": [
                        [
                          5582,
                          5586
                        ],
                        "func"
                      ]
                    },
                    {
                      "Ref": [
                        [
                          5587,
                          5588
                        ],
                        "a"
                      ]
                    }
                  ]
                }
              ],
              [
                {
                  "Adt": [
                    [
                      5594,
                      5601
                    ],
                    "Nothing",
                    []
                  ]
                },
                {
                  "Ref": [
                    [
                      5611,
                      5618
                    ],
                    "default"
                  ]
                }
              ]
            ]
          ]
        }
      }
    }
  ]
}